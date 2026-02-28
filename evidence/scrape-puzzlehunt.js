/**
 * MS Puzzlehunt Playwright scraper
 * Crawls puzzlehunt.azurewebsites.net and saves all puzzle data
 * Run: node scrape-puzzlehunt.js
 */

const { chromium } = require('playwright');
const fs = require('fs');
const path = require('path');

const OUT_DIR = path.join(__dirname, 'ph-scraped');
fs.mkdirSync(OUT_DIR, { recursive: true });

const BASE = 'https://puzzlehunt.azurewebsites.net';

const HUNTS = ['ph20', 'ph21', 'ph22', 'ph24'];

async function save(name, content) {
  fs.writeFileSync(path.join(OUT_DIR, name), content, 'utf8');
  console.log(`Saved: ${name} (${content.length} chars)`);
}

async function scrapeHunt(page, huntId) {
  console.log(`\n=== Scraping ${huntId} ===`);
  const results = { huntId, pages: {} };

  // Public pages
  const publicPaths = ['play', 'play/samples', 'play/Rules', 'play/faq', 'play/tools', 'play/encodings'];
  for (const p of publicPaths) {
    try {
      await page.goto(`${BASE}/${huntId}/${p}`, { waitUntil: 'domcontentloaded', timeout: 15000 });
      await page.waitForTimeout(1000);
      const text = await page.evaluate(() => document.body.innerText);
      const html = await page.content();
      results.pages[p] = { text: text.substring(0, 50000), url: `${BASE}/${huntId}/${p}` };
      console.log(`  ${p}: ${text.length} chars`);
    } catch (e) {
      console.log(`  ${p}: ERROR - ${e.message.substring(0, 80)}`);
    }
  }

  // Try puzzle list (requires auth - will get login page or actual data)
  const authPaths = ['play/Puzzles', 'play/Teams/All'];
  for (const p of authPaths) {
    try {
      await page.goto(`${BASE}/${huntId}/${p}`, { waitUntil: 'domcontentloaded', timeout: 15000 });
      await page.waitForTimeout(1500);
      const text = await page.evaluate(() => document.body.innerText);
      const url = page.url();
      const isLoginPage = url.includes('Login') || text.includes('Please login');
      results.pages[p] = { text: text.substring(0, 100000), url, isLoginPage };
      console.log(`  ${p}: ${text.length} chars ${isLoginPage ? '(LOGIN REQUIRED)' : '*** HAS DATA ***'}`);
    } catch (e) {
      console.log(`  ${p}: ERROR - ${e.message.substring(0, 80)}`);
    }
  }

  return results;
}

async function scrapePuzzleUniversity(page) {
  console.log('\n=== Scraping puzzle.university ===');
  const rounds = ['music', 'english', 'history', 'math', 'classics', 'sociology', 'computer-science', 'placement-test', 'econ'];
  const results = {};

  for (const round of rounds) {
    try {
      await page.goto(`https://puzzle.university/round/${round}.html`, { waitUntil: 'domcontentloaded', timeout: 15000 });
      await page.waitForTimeout(800);
      const text = await page.evaluate(() => document.body.innerText);
      const links = await page.evaluate(() =>
        Array.from(document.querySelectorAll('a[href]'))
          .map(a => ({ href: a.href, text: a.innerText.trim() }))
          .filter(a => a.href.includes('puzzle.university'))
      );
      results[round] = { text: text.substring(0, 80000), links };
      console.log(`  ${round}: ${text.length} chars, ${links.length} links`);
    } catch (e) {
      console.log(`  ${round}: ERROR - ${e.message.substring(0, 80)}`);
    }
  }

  // Scrape individual puzzle solution pages found from links
  const solutionLinks = [];
  for (const [round, data] of Object.entries(results)) {
    for (const link of (data.links || [])) {
      if (link.href.includes('/solution/')) solutionLinks.push(link.href);
    }
  }

  console.log(`\nFound ${solutionLinks.length} solution pages to scrape...`);
  const solutions = {};
  for (const url of solutionLinks.slice(0, 100)) { // cap at 100
    try {
      await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 12000 });
      await page.waitForTimeout(500);
      const text = await page.evaluate(() => document.body.innerText);
      const slug = url.split('/solution/')[1]?.replace('.html', '') || url;
      solutions[slug] = text.substring(0, 10000);
      process.stdout.write('.');
    } catch (e) {
      process.stdout.write('x');
    }
  }
  console.log('');

  return { rounds: results, solutions };
}

async function scrapeGMPuzzles(page) {
  console.log('\n=== Scraping gmpuzzles.com (Thomas Snyder) ===');
  const results = {};
  try {
    await page.goto('https://www.gmpuzzles.com/blog/', { waitUntil: 'domcontentloaded', timeout: 20000 });
    await page.waitForTimeout(1000);
    const text = await page.evaluate(() => document.body.innerText);
    const links = await page.evaluate(() =>
      Array.from(document.querySelectorAll('a[href*="gmpuzzles.com/blog/20"]'))
        .map(a => ({ href: a.href, text: a.innerText.trim() }))
        .slice(0, 30)
    );
    results.blog = { text: text.substring(0, 20000), recentLinks: links };
    console.log(`  blog: ${text.length} chars, ${links.length} recent posts`);
  } catch (e) {
    console.log(`  blog: ERROR - ${e.message.substring(0, 80)}`);
  }
  return results;
}

async function scrapePuzzlvaria(page) {
  console.log('\n=== Scraping puzzlvaria.wordpress.com (Dan Katz) ===');
  const posts = [
    'https://puzzlvaria.wordpress.com/2019/04/18/metapuzzles-backsolving-and-short-circuiting-a-study-of-three-puzzlehunts/',
    'https://puzzlvaria.wordpress.com/2025/01/05/unlockers-remorse-the-risks-and-rewards-of-choice-as-a-puzzlehunt-mechanism/',
    'https://puzzlvaria.wordpress.com/2026/01/19/2026-mit-mystery-hunt/',
    'https://puzzlvaria.wordpress.com/2025/07/01/a-collection-of-varicella-driven-thoughts-part-2/',
    'https://puzzlvaria.wordpress.com/2025/02/07/mit-mystery-hunt-2025-part-2-some-puzzles-i-worked-on/',
  ];
  const results = {};
  for (const url of posts) {
    try {
      await page.goto(url, { waitUntil: 'domcontentloaded', timeout: 15000 });
      await page.waitForTimeout(800);
      const text = await page.evaluate(() => document.body.innerText);
      const slug = url.split('/').filter(Boolean).pop();
      results[slug] = text.substring(0, 30000);
      console.log(`  ${slug}: ${text.length} chars`);
    } catch (e) {
      console.log(`  ERROR: ${e.message.substring(0, 80)}`);
    }
  }
  return results;
}

async function main() {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36' });
  const page = await context.newPage();

  const allData = {};

  // 1. Scrape puzzle.university (public)
  allData.puzzleUniversity = await scrapePuzzleUniversity(page);
  save('puzzle-university.json', JSON.stringify(allData.puzzleUniversity, null, 2));

  // 2. Scrape MS Puzzlehunt server (public pages + attempt auth pages)
  allData.msph = {};
  for (const huntId of HUNTS) {
    allData.msph[huntId] = await scrapeHunt(page, huntId);
    save(`msph-${huntId}.json`, JSON.stringify(allData.msph[huntId], null, 2));
  }

  // 3. Scrape GMPuzzles
  allData.gmpuzzles = await scrapeGMPuzzles(page);
  save('gmpuzzles.json', JSON.stringify(allData.gmpuzzles, null, 2));

  // 4. Scrape Puzzlvaria
  allData.puzzlvaria = await scrapePuzzlvaria(page);
  save('puzzlvaria.json', JSON.stringify(allData.puzzlvaria, null, 2));

  await browser.close();

  // Write summary
  const summary = [];
  summary.push('# Playwright Scrape Summary\n');
  summary.push(`## puzzle.university rounds scraped: ${Object.keys(allData.puzzleUniversity?.rounds || {}).join(', ')}`);
  summary.push(`## puzzle.university solutions scraped: ${Object.keys(allData.puzzleUniversity?.solutions || {}).length}`);
  for (const [huntId, data] of Object.entries(allData.msph || {})) {
    for (const [p, pageData] of Object.entries(data.pages || {})) {
      if (!pageData.isLoginPage && pageData.text?.length > 500) {
        summary.push(`## ${huntId}/${p}: HAS DATA (${pageData.text.length} chars)`);
      }
    }
  }
  save('SUMMARY.md', summary.join('\n'));
  console.log('\n=== DONE ===');
  console.log(`Files saved to: ${OUT_DIR}`);
}

main().catch(console.error);
