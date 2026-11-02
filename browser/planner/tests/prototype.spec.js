import { test, expect } from "@playwright/test";
import fs from "node:fs";
import { createHash } from "node:crypto";
const cases=JSON.parse(fs.readFileSync(new URL("../.cache/native-cases.json",import.meta.url),"utf8"));

test("real worker exactly repeats every native synthetic control outcome",async({page})=>{
  test.setTimeout(90000);await page.goto("./");await expect(page.locator("#run")).toBeEnabled();
  const outcomes=await page.evaluate(async(cases)=>{
    const worker=new Worker(new URL("worker.js",location.href),{type:"module"});
    await new Promise((resolve,reject)=>{worker.onmessage=({data})=>data.type==="ready"&&resolve();worker.onerror=reject;});
    let id=0;const call=text=>new Promise((resolve,reject)=>{const requestId=++id,timer=setTimeout(()=>reject(new Error("worker deadline")),10000);worker.onmessage=({data})=>{if(data.id!==requestId)return;clearTimeout(timer);if(data.type!=="result")reject(new Error(data.message));else resolve(data);};worker.postMessage({id:requestId,text});});
    const results=[];for(const c of cases){const a=await call(c.request),b=await call(c.request);if(a.report!==b.report)throw new Error("nonrepeatable WASM output");results.push({request:c.request,response:JSON.parse(a.report),elapsedMs:a.elapsedMs});}worker.terminate();return results;
  },cases);
  const retained=outcomes.map(({request,response})=>({request,response}));
  fs.writeFileSync(".cache/wasm-cases.json",JSON.stringify(retained,null,2)+"\n");
  fs.writeFileSync(".cache/worker-timings.json",JSON.stringify(outcomes.map(({elapsedMs,request,response})=>({elapsedMs,requestBytes:Buffer.byteLength(request),kind:response.kind})),null,2)+"\n");
  expect(retained).toEqual(cases);
  for(const {request,response:r}of retained.filter(c=>c.response.kind==="report")){
    expect(r.request_sha256).toBe(createHash("sha256").update(request).digest("hex"));
    for(const side of [r.baseline,r.candidate]){
      const sorted=side.runs.map(r=>r.total_minutes).sort((a,b)=>a-b);
      expect(side.summary.p95_minutes).toBe(sorted[Math.ceil(sorted.length*.95)-1]);
      expect(side.summary.average_minutes).toBe(sorted.reduce((a,b)=>a+b,0)/sorted.length);
      expect(side.summary.bottlenecks.reduce((a,b)=>a+b.runs,0)).toBe(sorted.length);
      for(const run of side.runs){expect(run.solved).toHaveLength(6);expect(run.team.actions).toBe(7);expect(run.solved.map(p=>p.id).sort()).toEqual(["P1","P2","P3","P4","P5","P6"]);const groups=[];for(let i=0;i<6;i+=run.team.parallelism)groups.push(Math.max(...run.solved.slice(i,i+run.team.parallelism).map(p=>p.minutes)));expect(run.total_minutes).toBe(groups.reduce((a,b)=>a+b,0)+run.meta_minutes);}
    }
  }
});

test("own assumptions, exact exports and native diagnostics invalidate stale results",async({page})=>{
  await page.goto("./");await expect(page.locator("#run")).toBeEnabled();await expect(page.locator("#input")).not.toHaveValue("");
  const own=JSON.parse(await page.locator("#input").inputValue());own.seed="own-reader";own.runs=3;own.variant="baseline";own.puzzles[0].difficulty=7;
  const text=JSON.stringify(own,null,2);await page.locator("#input").fill(text);await page.locator("#run").click();await expect(page.locator("#status")).toContainText("Forecast complete");
  const report=JSON.parse(await page.locator("#raw").textContent());expect(report.baseline).toEqual(report.candidate);await expect(page.locator("#classification")).toContainText("regressed");await expect(page.locator("#comparison-note")).toContainText("including ties");
  let event=page.waitForEvent("download");await page.locator("#request").click();expect(fs.readFileSync(await(await event).path(),"utf8")).toBe(text);
  event=page.waitForEvent("download");await page.locator("#report").click();expect(JSON.parse(fs.readFileSync(await(await event).path(),"utf8"))).toEqual(report);
  await page.locator("#input").fill('{"bad":"<img src=x onerror=alert(1)>"}');await expect(page.locator("#report")).toBeDisabled();await expect(page.locator("#results")).toBeHidden();await page.locator("#run").click();await expect(page.locator("#status")).toContainText("invalid_request");await expect(page.locator("#input")).toHaveValue('{"bad":"<img src=x onerror=alert(1)>"}');await expect(page.locator("img")).toHaveCount(0);
});

test("opened original stays byte exact while LF copy, mobile and keyboard work",async({page})=>{
  await page.setViewportSize({width:390,height:844});await page.goto("./");await expect(page.locator("#run")).toBeEnabled();await expect(page.locator("#input")).not.toHaveValue("");
  const text=await page.locator("#input").inputValue(),original=Buffer.from("\ufeff"+text.replaceAll("\n","\r\n"));
  await page.locator("#file").setInputFiles({name:"own.json",mimeType:"application/json",buffer:original});await expect(page.locator("#original")).toBeEnabled();
  const event=page.waitForEvent("download");await page.locator("#original").click();expect(fs.readFileSync(await(await event).path())).toEqual(original);
  await page.locator("#lf").click();expect((await page.locator("#input").inputValue()).startsWith("\ufeff")).toBe(false);await page.locator("#run").focus();await page.locator("#run").press("Enter");await expect(page.locator("#status")).toContainText("Forecast complete");expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
  await page.screenshot({path:".cache/mobile.png",fullPage:true});await page.setViewportSize({width:1280,height:900});await page.screenshot({path:".cache/desktop.png",fullPage:true});
});

test("unavailable WASM preserves assumptions and independent model/build routes",async({page,request})=>{
  await page.route("**/*.wasm",route=>route.abort());await page.goto("./");await expect(page.locator("#status")).toContainText(/failed|unavailable/i);await expect(page.locator("#input")).not.toHaveValue("");await expect(page.locator("#run")).toBeDisabled();
  for(const name of ["Build instructions","Model and custody details"]){const link=page.getByRole("link",{name,exact:true});expect((await request.get(await link.getAttribute("href"))).ok()).toBe(true);}
});
