#!/usr/bin/env python3
"""One bounded fresh directional smoke; never full acceptance or saved-state resume."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time

TEST = 'directional_ci_fresh_crossed_delivery'
TARGET = 'na0780_directional_integration'
LIMIT = 600


def run(command, path, env):
    started = time.monotonic()
    with path.open('xb') as log:
        child = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT,
                                 env=env, start_new_session=True)
        try:
            code = child.wait(timeout=LIMIT)
            return {'exit_code': code, 'status': 'passed' if code == 0 else 'failed',
                    'elapsed_seconds': time.monotonic() - started}
        except subprocess.TimeoutExpired:
            return {'exit_code': None, 'status': 'timeout',
                    'elapsed_seconds': time.monotonic() - started}
        finally:
            # Also clean descendants left after an unexpected parent exit.
            try:
                os.killpg(child.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
            try:
                child.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(child.pid, signal.SIGKILL)
                child.wait(timeout=5)
            try:
                os.killpg(child.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', required=True)
    args = parser.parse_args()
    def interrupted(signum, frame):
        raise InterruptedError('runner_signal')
    signal.signal(signal.SIGTERM, interrupted)
    out = Path(args.output).resolve()
    out.mkdir(mode=0o700)  # Exclusive: rerun never overwrites evidence/fixtures.
    env = os.environ.copy()
    env['CARGO_TERM_COLOR'] = 'never'
    env['QSC_TEST_ROOT'] = str(out / 'fresh-fixture')
    result = {'status': 'failed', 'test': TEST, 'target': TARGET,
              'scope': 'two fresh crossed-delivery rounds, not full acceptance',
              'compile_bound_seconds': LIMIT, 'test_bound_seconds': LIMIT,
              'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()}
    try:
        compile_result = run(['cargo', '+stable', 'test', '-p', 'qsc', '--locked',
                              '--features', 'na0780-test-hooks', '--test', TARGET,
                              '--no-run', '--message-format=json'], out / 'compile.log', env)
        (out / 'compile.json').write_text(json.dumps(compile_result, indent=2) + '\n')
        if compile_result['status'] != 'passed':
            result['reason'] = 'compile_' + compile_result['status']
            return 1
        artifacts = {}
        for line in (out / 'compile.log').read_text().splitlines():
            if not line.startswith('{'):
                continue
            message = json.loads(line)
            if message.get('reason') == 'compiler-artifact' and message.get('executable'):
                artifacts[message['target']['name']] = message['executable']
        binary = Path(artifacts[TARGET])
        result['test_binary_sha256'] = hashlib.sha256(binary.read_bytes()).hexdigest()
        test_result = run(['cargo', '+stable', 'test', '-p', 'qsc', '--locked',
                           '--features', 'na0780-test-hooks', '--test', TARGET,
                           TEST, '--', '--exact', '--test-threads=1', '--nocapture'],
                          out / 'test.log', env)
        result['test_result'] = test_result
        log = (out / 'test.log').read_text()
        print(log, end='')
        # Exact selection must execute one test and both orderings, never zero/ignored.
        expected = [f'NA0780_CI order={n} same_operation_delivered=2 result=pass' for n in (0, 1)]
        passed = (test_result['status'] == 'passed'
                  and all(log.count(marker) == 1 for marker in expected)
                  and log.count('test result: ok. 1 passed; 0 failed; 0 ignored;') == 1)
        result['status'] = 'passed' if passed else 'failed'
        result['reason'] = 'two_orderings_same_operation' if passed else 'exit_or_inventory_failure'
        return 0 if passed else 1
    except (Exception, KeyboardInterrupt) as error:
        result['reason'] = type(error).__name__
        return 1
    finally:
        (out / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
        print(json.dumps(result), flush=True)


if __name__ == '__main__':
    sys.exit(main())
