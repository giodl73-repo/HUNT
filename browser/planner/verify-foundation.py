"""Verify the scoped engine extraction against the retained native entry."""
import json,pathlib,re,subprocess
R=pathlib.Path(__file__).resolve().parents[2]
entry=subprocess.check_output(['git','rev-parse','edd0f6e'],cwd=R,text=True).strip()
original=subprocess.check_output(['git','show',entry+':tools/hunt-sim/src/main.rs'],cwd=R).decode()
main=(R/'tools/hunt-sim/src/main.rs').read_text(encoding='utf-8');core=(R/'tools/hunt-sim/core/src/lib.rs').read_text(encoding='utf-8')
def tokens(text):return re.findall(r'"(?:\\.|[^"\\])*"|[^\s]',text)
def function(text,name):
 start=text.index('fn '+name+'(');tail=tokens(text[start:]);begin=tail.index('{');depth=0
 for i,t in enumerate(tail[begin:],begin):
  if t=='{':depth+=1
  elif t=='}':
   depth-=1
   if depth==0:return tail[:i+1]
 raise AssertionError(name)
for name in ['main','run_game_night_cli','has_flag','option_value','find_variant','find_game_night_variant','simulate_game_night_batch','summarize_game_night_batch','compare_game_night_to_baseline','simulate_game_night','solve_game_night_module','simulate_batch']:
 assert function(original,name)==function(main,name),name
for name,renamed in [('simulate_wavelength','simulate_feeders'),('summarize_batch','summarize_feeders'),('compare_to_baseline','compare_to_baseline'),('solve_puzzle','solve_puzzle')]:
 expected=' '.join(function(original,name))
 expected=expected.replace(' '.join(name),' '.join(renamed),1)
 # Signature-only injection and the explicit native constant -> caller data binding.
 if name=='simulate_wavelength':
  expected=expected.replace(' '.join('seed:&str,'),' '.join('seed:&str,puzzles:&[Puzzle],'),1)
 elif name=='summarize_batch':
  expected=expected.replace(' '.join('('),' '.join('(puzzles:&[Puzzle],'),1)
 expected=expected.replace(' '.join('WAVELENGTH'),' '.join('puzzles'))
 left=tokens(expected);right=function(core,renamed)
 # rustfmt introduces a trailing argument comma when signatures wrap.
 for value in [left,right]:
  opening=value.index('{')
  for i in range(opening-1,0,-1):
   if value[i]==')' and value[i-1]==',':value.pop(i-1)
 assert left==right,name
for name,next_name in [('BASELINE','VARIANTS'),('VARIANTS','GAME_NIGHT_BASELINE')]:
 old=original[original.index('const '+name+':'):original.index('const '+next_name+':')].strip()
 start=core.index('pub const '+name+':');end=core.find('pub const ',start+5);new=core[start:end if end!=-1 else None].strip()
 assert tokens(old)==tokens(new.removeprefix('pub ')),name
assert all(x not in core for x in ['const WAVELENGTH','const GAME_NIGHT_MODULES','answer:', 'std::env'])
for file in ['tools/hunt-sim/Cargo.lock','tools/hunt-sim/core/Cargo.lock','browser/planner/Cargo.lock']:
 data=(R/file).read_text(encoding='utf-8');assert 'f64d8d4a5a04d8890ebcb9448ce28eb4f25861e7' in data
for path in ['README.md','LICENSE','BUGS.md','docs/pitfall-boundaries.v1.json','tests/check-pitfall-policy.ps1']:
 expected=subprocess.check_output(['git','rev-parse',entry+':'+path],cwd=R).strip()
 current=subprocess.check_output(['git','hash-object','--path',path,path],cwd=R).strip()
 raw=subprocess.check_output(['git','hash-object','--no-filters',path],cwd=R).strip()
 assert expected in [current,raw],path
print(json.dumps({'entry':entry,'native_functions_unchanged':12,'shared_functions_token_exact_except_explicit_bindings':4,'native_variant_constants_exact':2,'rally_revision':'f64d8d4a5a04d8890ebcb9448ce28eb4f25861e7','protected_policy_and_grants':'exact'}))
