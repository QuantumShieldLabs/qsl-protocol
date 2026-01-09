.PHONY: venv 4a clean

venv:
	python3 -m venv .venv
	. .venv/bin/activate && pip install -r requirements-ci.txt

4a:
	scripts/ci/run_4a.sh

clean:
	rm -rf artifacts evidence specs/_extracted
