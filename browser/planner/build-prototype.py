"""Build the real native/WASM prototype and private synthetic control outcomes."""
import argparse,hashlib,json,os,pathlib,shutil,subprocess
p=argparse.ArgumentParser();p.add_argument('--bindgen',required=True,type=pathlib.Path);a=p.parse_args()
B=pathlib.Path(__file__).resolve().parent;R=B.parents[1];env=os.environ.copy();target=pathlib.Path(env.get('CARGO_TARGET_DIR',str(R/'target/cargo'))).resolve();env['CARGO_TARGET_DIR']=str(target)
env['CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS']='-C link-arg=--max-memory=67108864'
def run(args):subprocess.run([str(x)for x in args],cwd=R,env=env,check=True)
run(['cargo','+1.95.0','build','--locked','--manifest-path',B/'Cargo.toml','--release'])
run(['cargo','+1.95.0','build','--locked','--manifest-path',B/'Cargo.toml','--release','--target','wasm32-unknown-unknown'])
site=R/'target/hunt-prototype/HUNT';assert site.resolve()==R/'target/hunt-prototype/HUNT'
if site.exists():shutil.rmtree(site)
shutil.copytree(B/'web',site);run([a.bindgen.resolve(),target/'wasm32-unknown-unknown/release/hunt_planner.wasm','--target','web','--out-dir',site/'pkg'])
for generated in (site/'pkg').glob('*.d.ts'):generated.unlink()
for name in ['BUILD.md','MODEL.md']:shutil.copyfile(B/name,site/name)
example=json.loads((B/'web/example.json').read_bytes());cases=[]
for variant in ['baseline','stronger-hints','p5-p6-clue-relief','meta-prop-clarity','team-parallelism','guided-final-set']:
 for seed in ['', 'synthetic-reader','own-α']:
  for skill,parallelism in [(0,1),(3,3)]:
   case=json.loads(json.dumps(example));case.update(variant=variant,seed=seed,runs=3);case['team']={'skill':skill,'parallelism':parallelism};cases.append(case)
maximum=json.loads(json.dumps(example));maximum['runs']=128;maximum['team']={'skill':10,'parallelism':6};maximum['puzzles']=[{'difficulty':10,'answer_length':32}for _ in range(6)];cases.append(maximum)
for key,value in [('runs',0),('runs',129),('runs',1.5),('puzzles',[]),('seed','x'*129),('variant','unknown'),('answer','no answer input')]:
 case=json.loads(json.dumps(example));case[key]=value;cases.append(case)
cases.extend(['{',' '*65_537]);records=[];exe=target/'release'/('hunt-planner-cli.exe'if os.name=='nt'else'hunt-planner-cli')
for c in cases:
 text=c if isinstance(c,str)else json.dumps(c,ensure_ascii=False,separators=(',',':'));outputs=[subprocess.check_output([str(exe)],input=text.encode(),env=env)for _ in range(2)];assert outputs[0]==outputs[1];records.append({'request':text,'response':json.loads(outputs[0])})
cache=B/'.cache';cache.mkdir(exist_ok=True);(cache/'native-cases.json').write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
wasm=site/'pkg/hunt_planner_bg.wasm';data=wasm.read_bytes();print(json.dumps({'site':str(site),'native_cases':len(records),'wasm_bytes':len(data),'wasm_sha256':hashlib.sha256(data).hexdigest()}))
