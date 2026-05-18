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

function clipText(text, limit) {
  return text.substring(0, limit);
}

function logScrapeError(label, e, writer = console.log) {
  writer(`  ${label}: ERROR - ${e.message.substring(0, 80)}`);
}

async function loadPageText(page, url, timeout, settleMs) {
  await page.goto(url, { waitUntil: 'domcontentloaded', timeout });
  await page.waitForTimeout(settleMs);
  return page.evaluate(() => document.body.innerText);
}

async function scrapeTextPage(page, label, url, options) {
  try {
    const text = await loadPageText(page, url, options.timeout, options.settleMs);
    return { ok: true, text, url: page.url() };
  } catch (e) {
    logScrapeError(label, e);
    return { ok: false };
  }
}

async function scrapeHunt(page, huntId) {
  console.log(`\n=== Scraping ${huntId} ===`);
  const results = { huntId, pages: {} };

  await scrapeHuntPages(page, huntId, results, publicHuntPaths(), scrapePublicHuntPage);
  await scrapeHuntPages(page, huntId, results, authHuntPaths(), scrapeAuthHuntPage);

  return results;
}

function publicHuntPaths() {
  return ['play', 'play/samples', 'play/Rules', 'play/faq', 'play/tools', 'play/encodings'];
}

function authHuntPaths() {
  return ['play/Puzzles', 'play/Teams/All'];
}

async function scrapeHuntPages(page, huntId, results, paths, handler) {
  for (const p of paths) await handler(page, huntId, results, p);
}

async function scrapePublicHuntPage(page, huntId, results, p) {
  const url = `${BASE}/${huntId}/${p}`;
  const scraped = await scrapeTextPage(page, p, url, { timeout: 15000, settleMs: 1000 });
  if (!scraped.ok) return;
  results.pages[p] = { text: clipText(scraped.text, 50000), url };
  console.log(`  ${p}: ${scraped.text.length} chars`);
}

async function scrapeAuthHuntPage(page, huntId, results, p) {
  const scraped = await scrapeTextPage(page, p, `${BASE}/${huntId}/${p}`, { timeout: 15000, settleMs: 1500 });
  if (!scraped.ok) return;
  const isLoginPage = scraped.url.includes('Login') || scraped.text.includes('Please login');
  results.pages[p] = { text: clipText(scraped.text, 100000), url: scraped.url, isLoginPage };
  console.log(`  ${p}: ${scraped.text.length} chars ${isLoginPage ? '(LOGIN REQUIRED)' : '*** HAS DATA ***'}`);
}

async function collectPuzzleUniversityLinks(page) {
  return page.evaluate(() =>
    Array.from(document.querySelectorAll('a[href]'))
      .map(a => ({ href: a.href, text: a.innerText.trim() }))
      .filter(a => a.href.includes('puzzle.university'))
  );
}

async function scrapePuzzleUniversity(page) {
  console.log('\n=== Scraping puzzle.university ===');
  const results = {};

  for (const round of puzzleUniversityRounds()) await scrapePuzzleUniversityRound(page, results, round);

  const solutions = await scrapePuzzleUniversitySolutions(page, solutionLinksFromRounds(results));
  return { rounds: results, solutions };
}

function puzzleUniversityRounds() {
  return ['music', 'english', 'history', 'math', 'classics', 'sociology', 'computer-science', 'placement-test', 'econ'];
}

async function scrapePuzzleUniversityRound(page, results, round) {
  const url = `https://puzzle.university/round/${round}.html`;
  const scraped = await scrapeTextPage(page, round, url, { timeout: 15000, settleMs: 800 });
  if (!scraped.ok) return;
  const links = await collectPuzzleUniversityLinks(page);
  results[round] = { text: clipText(scraped.text, 80000), links };
  console.log(`  ${round}: ${scraped.text.length} chars, ${links.length} links`);
}

function solutionLinksFromRounds(results) {
  const solutionLinks = [];
  for (const data of Object.values(results)) collectSolutionLinks(solutionLinks, data.links || []);
  return solutionLinks;
}

function collectSolutionLinks(solutionLinks, links) {
  for (const link of links) {
    if (link.href.includes('/solution/')) solutionLinks.push(link.href);
  }
}

async function scrapePuzzleUniversitySolutions(page, solutionLinks) {
  console.log(`\nFound ${solutionLinks.length} solution pages to scrape...`);
  const solutions = {};
  for (const url of solutionLinks.slice(0, 100)) { // cap at 100
    await scrapePuzzleUniversitySolution(page, solutions, url);
  }
  console.log('');
  return solutions;
}

async function scrapePuzzleUniversitySolution(page, solutions, url) {
  try {
    const text = await loadPageText(page, url, 12000, 500);
    const slug = url.split('/solution/')[1]?.replace('.html', '') || url;
    solutions[slug] = clipText(text, 10000);
    process.stdout.write('.');
  } catch (e) {
    process.stdout.write('x');
  }
}

async function scrapeGMPuzzles(page) {
  console.log('\n=== Scraping gmpuzzles.com (Thomas Snyder) ===');
  const results = {};
  try {
    const text = await loadPageText(page, 'https://www.gmpuzzles.com/blog/', 20000, 1000);
    const links = await page.evaluate(() =>
      Array.from(document.querySelectorAll('a[href*="gmpuzzles.com/blog/20"]'))
        .map(a => ({ href: a.href, text: a.innerText.trim() }))
        .slice(0, 30)
    );
    results.blog = { text: clipText(text, 20000), recentLinks: links };
    console.log(`  blog: ${text.length} chars, ${links.length} recent posts`);
  } catch (e) {
    logScrapeError('blog', e);
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
      const text = await loadPageText(page, url, 15000, 800);
      const slug = url.split('/').filter(Boolean).pop();
      results[slug] = clipText(text, 30000);
      console.log(`  ${slug}: ${text.length} chars`);
    } catch (e) {
      logScrapeError('ERROR', e);
    }
  }
  return results;
}

async function main() {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ userAgent: 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36' });
  const page = await context.newPage();

  const allData = await scrapeAllData(page);
  await browser.close();
  save('SUMMARY.md', buildSummary(allData));
  console.log('\n=== DONE ===');
  console.log(`Files saved to: ${OUT_DIR}`);
}

async function scrapeAllData(page) {
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

  return allData;
}

function buildSummary(allData) {
  const summary = [];
  summary.push('# Playwright Scrape Summary\n');
  summary.push(`## puzzle.university rounds scraped: ${puzzleUniversityRoundNames(allData)}`);
  summary.push(`## puzzle.university solutions scraped: ${puzzleUniversitySolutionCount(allData)}`);
  appendMsphSummary(summary, msphHunts(allData));
  return summary.join('\n');
}

function puzzleUniversityData(allData) {
  return allData.puzzleUniversity || {};
}

function puzzleUniversityRoundNames(allData) {
  return Object.keys(puzzleUniversityData(allData).rounds || {}).join(', ');
}

function puzzleUniversitySolutionCount(allData) {
  return Object.keys(puzzleUniversityData(allData).solutions || {}).length;
}

function msphHunts(allData) {
  return allData.msph || {};
}

function appendMsphSummary(summary, hunts) {
  for (const [huntId, data] of Object.entries(hunts)) appendHuntSummary(summary, huntId, data);
}

function appendHuntSummary(summary, huntId, data) {
  for (const [p, pageData] of Object.entries(data.pages || {})) {
    appendPageSummary(summary, huntId, p, pageData);
  }
}

function appendPageSummary(summary, huntId, p, pageData) {
  if (!hasPageSummaryData(pageData)) return;
  summary.push(`## ${huntId}/${p}: HAS DATA (${pageData.text.length} chars)`);
}

function hasPageSummaryData(pageData) {
  if (pageData.isLoginPage) return false;
  return pageTextLength(pageData) > 500;
}

function pageTextLength(pageData) {
  return (pageData.text || '').length;
}

main().catch(console.error);
