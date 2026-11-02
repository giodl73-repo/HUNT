"""Repeat original CLI controls; retain hashes only, never print scenario output."""
import hashlib,json,os,pathlib,subprocess
N=pathlib.Path(__file__).resolve().parent;R=N.parents[1];manifest=N/'Cargo.toml'
subprocess.run(['cargo','+1.95.0','build','--locked','--manifest-path',str(manifest)],cwd=R,check=True)
metadata=json.loads(subprocess.check_output(['cargo','+1.95.0','metadata','--locked','--no-deps','--format-version','1','--manifest-path',str(manifest)],cwd=R))
exe=pathlib.Path(metadata['target_directory'])/'debug'/('hunt-sim.exe'if os.name=='nt'else'hunt-sim')
record=json.loads((R/'context/waves/2026-11-01-hunt-browser/portability.json').read_bytes());cases=record['native_cli_case_pins'];assert len(cases)==48
for index,case in enumerate(cases):
 for repeat in range(2):
  output=subprocess.check_output([str(exe),*case['args']],cwd=R)
  assert hashlib.sha256(output).hexdigest()==case['sha256'],f'Native control {index} differs (output withheld for answer custody)'
print(json.dumps({'original_native_cli_controls_exact':len(cases),'each_repeated':2,'raw_scenario_output_published':False}))
