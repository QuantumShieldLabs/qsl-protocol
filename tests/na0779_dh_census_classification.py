#!/usr/bin/env python3
"""Exercise the actual DH census in a disposable tracked-tree copy.

Run after selecting the repository's existing qbuild Cargo environment:
python3 tests/na0779_dh_census_classification.py --repo REPO --output NEW_EVIDENCE_DIRECTORY
The output directory must be outside REPO. Requires git and the existing Cargo
installation; never installs tools or changes the source repository.
"""
import argparse
import hashlib
import json
import os
import re
from pathlib import Path
import shutil
import subprocess

# Assertions are this runner's checks; optimized Python must never disable them.
if not __debug__:
    raise RuntimeError("run without Python optimization so all verification checks execute")

TEST = 'suite2::ratchet::tests::na0628_every_dh_call_site_is_guarded_or_allowlisted'
SOURCE = Path('tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs')
ARCHIVE = Path('docs/audits/2026-09-03/AUDIT_harness_exp.rs')
MANIFEST = Path('tools/refimpl/quantumshield_refimpl/Cargo.toml')
ENTRY = '''        AllowedUnguardedDh {
            file: "docs/audits/2026-09-03/AUDIT_harness_exp.rs",
            function: "establish",
            reason: "archived historical state-machine harness using ToyDh to construct a matched pair; not production X25519 execution and not evidence of cryptographic strength.",
        },
'''.encode()
PIN = b'        ("docs/audits/2026-09-03/AUDIT_harness_exp.rs", 2),\n'
CALL = b'    let dh_init = dh.dh(&a_priv, &b_pub);\n'


def digest(data):
    return hashlib.sha256(data).hexdigest()


def verify_case_output(name, returncode, raw_output, expected, archive):
    """Require the named Rust test and exact failure mechanism, not just nonzero exit."""
    text = raw_output.decode(errors='replace')
    assert 'running 1 test' in text, (name, 'test did not execute')
    if expected == 'pass':
        assert returncode == 0 and '1 passed; 0 failed' in text, name
        assert f'test {TEST} ... ok' in text, (name, 'wrong test')
        return
    assert returncode == 101 and '0 passed; 1 failed' in text, (name, 'not a Cargo test failure')
    assert f'test {TEST} ... FAILED' in text, (name, 'wrong failing test')
    assert expected in text, (name, 'wrong failure reason')
    if name == 'removed-classification':
        # Both historical calls, and only those calls, must be identified as unguarded.
        start = text.index('unguarded X25519 DH output at:')
        block = text[start:].split('\n\n', 1)[0].splitlines()[1:]
        expected_sites = [f'{ARCHIVE}:{i} (fn establish)' for i, line in
                          enumerate(archive.decode().splitlines(), 1) if '.dh(' in line]
        assert len(expected_sites) == 2
        assert [line.strip() for line in block] == expected_sites, (name, 'wrong unguarded sites')
    elif name == 'extra-call':
        assert 'unguarded X25519 DH output at:' not in text
        found = re.search(r'^\s*left: (\{.*\})$', text, re.MULTILINE)
        pinned = re.search(r'^\s*right: (\{.*\})$', text, re.MULTILINE)
        assert found and pinned, (name, 'count comparison absent')
        found, pinned = json.loads(found[1]), json.loads(pinned[1])
        assert found[str(ARCHIVE)] == 3 and pinned[str(ARCHIVE)] == 2
        found[str(ARCHIVE)] = 2
        assert found == pinned, (name, 'unexpected additional count drift')
    else:
        raise AssertionError((name, 'unknown negative case'))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    repo, output = args.repo.resolve(), args.output.resolve()
    assert not output.is_relative_to(repo), 'fixture must be outside the scanned source tree'
    target = Path(os.environ['CARGO_TARGET_DIR']).resolve()
    assert not target.is_relative_to(output) and not target.is_relative_to(repo), 'use the selected shared Cargo target'
    output.mkdir(parents=True, exist_ok=False)
    fixture = output / 'fixture'
    names = subprocess.check_output(['git', 'ls-files', '-z'], cwd=repo).decode().split('\0')
    originals = {}
    for name in filter(None, names):
        src, dst = repo / name, fixture / name
        # Refuse links: the fixture must never write through to an original.
        assert not src.is_symlink(), f'unsupported tracked symlink: {name}'
        originals[name] = digest(src.read_bytes())
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dst)
    source, archive = (fixture / SOURCE).read_bytes(), (fixture / ARCHIVE).read_bytes()
    assert source.count(ENTRY) == 1 and source.count(PIN) == 1
    assert archive.count(CALL) == 1
    results = []

    def run_case(name, source_bytes, archive_bytes, expected):
        (fixture / SOURCE).write_bytes(source_bytes)
        (fixture / ARCHIVE).write_bytes(archive_bytes)
        command = ['cargo', 'test', '--locked', '--manifest-path', str(MANIFEST),
                   '--lib', TEST, '--', '--exact',
                   '--nocapture']
        result = subprocess.run(command, cwd=fixture, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        (output / (name + '.log')).write_bytes(result.stdout)
        verify_case_output(name, result.returncode, result.stdout, expected, archive)
        results.append({'case': name, 'exit': result.returncode, 'expected': expected, 'verified': True})
        print(name + ': expected result verified', flush=True)

    try:
        run_case('classified', source, archive, 'pass')
        run_case('removed-classification', source.replace(ENTRY, b''), archive,
                 'unguarded X25519 DH output at:')
        run_case('extra-call', source, archive.replace(CALL, CALL + b'    let _extra_mock_dh = dh.dh(&a_priv, &b_pub);\n'),
                 'the set of `.dh(` call sites changed')
        run_case('restored', source, archive, 'pass')
    finally:
        (fixture / SOURCE).write_bytes(source)
        (fixture / ARCHIVE).write_bytes(archive)
        assert all(digest((repo / name).read_bytes()) == sha for name, sha in originals.items()), 'original changed'
        (output / 'results.json').write_text(json.dumps({'cases': results, 'original_files_unchanged': len(originals),
            'archive_sha256': digest(archive), 'source_sha256': digest(source)}, indent=2) + '\n')


if __name__ == '__main__':
    main()
