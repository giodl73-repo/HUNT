"""Rebuild the delivered portable subset outside Git with exact per-target replay."""
import argparse,hashlib,json,os,pathlib,subprocess,tempfile,zipfile
p=argparse.ArgumentParser();p.add_argument('--site',required=True,type=pathlib.Path);p.add_argument('--bindgen',required=True,type=pathlib.Path);a=p.parse_args();B=pathlib.Path(__file__).resolve().parent;R=B.parents[1];C=B/'.cache';S=a.site.resolve();native=json.loads((C/'native-cases.json').read_bytes());wasm=json.loads((C/'wasm-cases.json').read_bytes());assert len(native)==len(wasm)==46
env=os.environ.copy()
for key in ['HUNT_BASE_URL','HUNT_EXPECTED_COMMIT','HUNT_SITE_DIR']:env.pop(key,None)
env['HUNT_SITE_DIR']='../../target/hunt-prototype'
log=C/'source-replay.log'
with log.open('wb')as output,tempfile.TemporaryDirectory(prefix='hunt source replay ')as temp:
 T=pathlib.Path(temp).resolve()
 with zipfile.ZipFile(S/'source.zip')as archive:
  for name in archive.namelist():
   path=pathlib.PurePosixPath(name);assert path.parts[0]=='HUNT-portable-source'and '..'not in path.parts and not path.is_absolute()
   target=T.joinpath(*path.parts);assert target.resolve().is_relative_to(T);target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(archive.read(name))
 W=T/'HUNT-portable-source';P=W/'browser/planner';assert ' 'in str(W)and not(W/'tools/hunt-sim/src/main.rs').exists();assert subprocess.run(['git','rev-parse','--show-toplevel'],cwd=W,capture_output=True).returncode!=0
 def run(args,cwd=W):
  result=subprocess.run([str(x)for x in args],cwd=cwd,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT);output.write(result.stdout);output.flush();assert result.returncode==0,result.stdout.decode(errors='replace')[-3000:];return result.stdout
 tests=run(['cargo','+1.95.0','test','--locked','--manifest-path',P/'Cargo.toml']);assert b'4 passed; 0 failed' in tests
 run(['python',P/'build-prototype.py','--bindgen',a.bindgen.resolve()]);rebuilt=json.loads((P/'.cache/native-cases.json').read_bytes());assert rebuilt==native,'Rebuilt native response set differs'
 npm='npm.cmd'if os.name=='nt'else'npm';run([npm,'ci'],cwd=P);run([npm,'run','test:pages','--','--grep-invert','packaged delivery|source/legal'],cwd=P)
 stats=json.loads((P/'.cache/browser-results.json').read_bytes())['stats'];assert stats['expected']==12 and all(stats[k]==0 for k in ['skipped','unexpected','flaky']);rebuilt_wasm=json.loads((P/'.cache/wasm-cases.json').read_bytes());assert rebuilt_wasm==wasm,'Rebuilt WASM responses differ'
 source_wasm=W/'target/hunt-prototype/HUNT/pkg/hunt_planner_bg.wasm';record={'outside_git':True,'path_contains_spaces':True,'native_scenario_files_absent':True,'adapter_tests':4,'browser_checks':12,'identical_rebuilt_native_responses':len(native),'identical_rebuilt_wasm_responses':len(wasm),'rebuilt_wasm_sha256':hashlib.sha256(source_wasm.read_bytes()).hexdigest(),'original_wasm_sha256':hashlib.sha256((S/'pkg/hunt_planner_bg.wasm').read_bytes()).hexdigest(),'binary_identity_claimed':False,'source_scope':'complete buildable portable software subset; external pinned dependencies are not vendored'}
(C/'source-replay.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8');print(json.dumps(record))
