"""Fail prototype artifact upload if native known answers or extra assets enter it."""
import json,pathlib,re
R=pathlib.Path(__file__).resolve().parents[2];S=R/'target/hunt-prototype/HUNT'
expected={'index.html','app.js','style.css','worker.js','example.json','BUILD.md','MODEL.md','pkg/hunt_planner.js','pkg/hunt_planner_bg.wasm'}
actual={p.relative_to(S).as_posix()for p in S.rglob('*')if p.is_file()}
assert actual==expected,{'extra':sorted(actual-expected),'missing':sorted(expected-actual)}
answers=re.findall(r'\banswer:\s*"([^"]+)"',(R/'tools/hunt-sim/src/main.rs').read_text(encoding='utf-8'));assert len(answers)==5
for p in S.rglob('*'):
 assert not p.is_symlink(),p
 if p.is_file():
  data=p.read_bytes()
  for answer in answers:assert re.search(rb'(?<![A-Za-z])'+re.escape(answer.encode())+rb'(?![A-Za-z])',data) is None,p.relative_to(S)
print(json.dumps({'prototype_files':len(actual),'known_native_answers_checked':len(answers),'scenario_content_included':False,'source_zip_qualified':False}))
