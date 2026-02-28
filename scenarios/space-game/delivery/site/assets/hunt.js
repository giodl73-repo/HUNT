/**
 * DEAD RECKONING — Hunt JavaScript
 * Answer checking, state management, unlock logic.
 * No external dependencies. Uses Web Crypto API for SHA-256.
 * localStorage key: dr_solved_puzzles
 */

'use strict';

// ============================================================
// CONSTANTS
// ============================================================

const STORAGE_KEY = 'dr_solved_puzzles';
const DATA_PATH   = 'data/puzzles.json'; // relative to site root

// Unlock rules. Each entry defines what must be solved before this puzzle unlocks.
// 'start' = always unlocked (no prerequisites)
const UNLOCK_RULES = {
  'R1-01':   { type: 'start' },
  'R1-02':   { type: 'start' },
  'R1-03':   { type: 'start' },
  'R1-04':   { type: 'start' },
  'R1-05':   { type: 'start' },
  'R1-META': { type: 'count', pool: ['R1-01','R1-02','R1-03','R1-04','R1-05'], threshold: 4 },
  'R2-01':   { type: 'solved', requires: ['R1-META'] },
  'R2-02':   { type: 'solved', requires: ['R1-META'] },
  'R2-03':   { type: 'solved', requires: ['R1-META'] },
  'R2-04':   { type: 'solved', requires: ['R1-META'] },
  'R2-05':   { type: 'solved', requires: ['R1-META'] },
  'R2-META': { type: 'count', pool: ['R2-01','R2-02','R2-03','R2-04','R2-05'], threshold: 4 },
  'R3-01':   { type: 'solved', requires: ['R2-META'] },
  'R3-02':   { type: 'solved', requires: ['R2-META'] },
  'R3-03':   { type: 'solved', requires: ['R2-META'] },
  'R3-04':   { type: 'solved', requires: ['R2-META'] },
  'R3-05':   { type: 'solved', requires: ['R2-META'] },
  'R3-META': { type: 'count', pool: ['R3-01','R3-02','R3-03','R3-04','R3-05'], threshold: 4 },
  'FINAL':   { type: 'solved', requires: ['R3-META'] },
};

// ============================================================
// STATE — localStorage-backed solved set
// ============================================================

/**
 * Return the set of solved puzzle IDs from localStorage.
 * @returns {Set<string>}
 */
function getSolvedPuzzles() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return new Set();
    return new Set(JSON.parse(raw));
  } catch {
    return new Set();
  }
}

/**
 * Persist the solved set back to localStorage.
 * @param {Set<string>} solved
 */
function saveSolvedPuzzles(solved) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify([...solved]));
  } catch {
    // localStorage unavailable — silently fail
  }
}

/**
 * Mark a puzzle as solved.
 * @param {string} puzzleId
 */
function markSolved(puzzleId) {
  const solved = getSolvedPuzzles();
  solved.add(puzzleId);
  saveSolvedPuzzles(solved);
}

/**
 * Check whether a puzzle ID has been solved.
 * @param {string} puzzleId
 * @returns {boolean}
 */
function isSolved(puzzleId) {
  return getSolvedPuzzles().has(puzzleId);
}

// ============================================================
// UNLOCK LOGIC
// ============================================================

/**
 * Determine whether a given puzzle is currently unlocked.
 * @param {string} puzzleId
 * @returns {boolean}
 */
function isPuzzleUnlocked(puzzleId) {
  const rule = UNLOCK_RULES[puzzleId];
  if (!rule) return false; // unknown puzzle — locked

  const solved = getSolvedPuzzles();

  switch (rule.type) {
    case 'start':
      return true;

    case 'solved':
      // All listed puzzles must be solved
      return rule.requires.every(id => solved.has(id));

    case 'count':
      // At least `threshold` puzzles from pool must be solved
      const solvedCount = rule.pool.filter(id => solved.has(id)).length;
      return solvedCount >= rule.threshold;

    default:
      return false;
  }
}

/**
 * Get progress for a round.
 * @param {number} roundNumber  — 1, 2, 3, or 0 for FINAL
 * @returns {{ solved: number, total: number, ids: string[] }}
 */
function getRoundProgress(roundNumber) {
  const allIds = Object.keys(UNLOCK_RULES);
  let ids;

  if (roundNumber === 1) {
    ids = ['R1-01','R1-02','R1-03','R1-04','R1-05','R1-META'];
  } else if (roundNumber === 2) {
    ids = ['R2-01','R2-02','R2-03','R2-04','R2-05','R2-META'];
  } else if (roundNumber === 3) {
    ids = ['R3-01','R3-02','R3-03','R3-04','R3-05','R3-META'];
  } else if (roundNumber === 0) {
    ids = ['FINAL'];
  } else {
    ids = allIds;
  }

  const solved = getSolvedPuzzles();
  const solvedCount = ids.filter(id => solved.has(id)).length;

  return { solved: solvedCount, total: ids.length, ids };
}

// ============================================================
// ANSWER CHECKING — Web Crypto SHA-256
// ============================================================

/**
 * Compute the SHA-256 hex digest of a string.
 * @param {string} text
 * @returns {Promise<string>} hex digest
 */
async function sha256(text) {
  const encoder = new TextEncoder();
  const data = encoder.encode(text);
  const hashBuffer = await crypto.subtle.digest('SHA-256', data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

/** Cached puzzles data — fetched once */
let _puzzlesData = null;

/**
 * Fetch and cache puzzles.json.
 * Resolves path relative to site root using a base tag or window.location.
 * @returns {Promise<object[]>}
 */
async function loadPuzzlesData() {
  if (_puzzlesData) return _puzzlesData;

  // Determine base URL — works from any subdirectory by resolving from root
  const base = getSiteRoot();
  const url = base + DATA_PATH;

  const response = await fetch(url);
  if (!response.ok) throw new Error(`Failed to load puzzles.json: ${response.status}`);
  const json = await response.json();
  _puzzlesData = json.puzzles;
  return _puzzlesData;
}

/**
 * Infer the site root from the current page's location.
 * Assumes all HTML pages are at most one level deep (e.g., admin/admin.html).
 * Returns a path ending with '/'.
 * @returns {string}
 */
function getSiteRoot() {
  const path = window.location.pathname;
  // Count depth of page relative to site root
  // index.html, puzzles.html, etc. → root
  // admin/admin.html → one level up
  const depth = (path.match(/\//g) || []).length - 1;
  if (depth <= 0) return './';
  return '../'.repeat(depth);
}

/**
 * Check a submitted answer against the stored hash for a puzzle.
 * @param {string} puzzleId    — e.g. "R1-01"
 * @param {string} input       — raw user input
 * @returns {Promise<{ correct: boolean, puzzleId: string, flavor: string|null }>}
 */
async function checkAnswer(puzzleId, input) {
  const normalized = input.toUpperCase().trim();
  const hash = await sha256(normalized);

  const puzzles = await loadPuzzlesData();
  const puzzle = puzzles.find(p => p.id === puzzleId);

  if (!puzzle) {
    return { correct: false, puzzleId, flavor: null };
  }

  // answerHash === 'TODO' means hashes not yet computed — never match
  if (!puzzle.answerHash || puzzle.answerHash === 'TODO') {
    return { correct: false, puzzleId, flavor: null };
  }

  const correct = hash === puzzle.answerHash;

  if (correct) {
    markSolved(puzzleId);
  }

  return { correct, puzzleId, flavor: correct ? (puzzle.solvedFlavor || null) : null };
}

// ============================================================
// DOM HELPERS
// ============================================================

/**
 * Update a single puzzle card element to reflect current state.
 * Expects card element to have data-puzzle-id attribute.
 * @param {HTMLElement} card
 */
function updatePuzzleCard(card) {
  const id = card.dataset.puzzleId;
  if (!id) return;

  const unlocked = isPuzzleUnlocked(id);
  const solved   = isSolved(id);

  // Remove all state classes first
  card.classList.remove('puzzle-card--locked', 'puzzle-card--unlocked', 'puzzle-card--solved');

  if (solved) {
    card.classList.add('puzzle-card--solved');
  } else if (unlocked) {
    card.classList.add('puzzle-card--unlocked');
  } else {
    card.classList.add('puzzle-card--locked');
  }

  // Update status text
  const statusEl = card.querySelector('.puzzle-status');
  if (statusEl) {
    statusEl.classList.remove('puzzle-status--locked','puzzle-status--unlocked','puzzle-status--solved');
    if (solved) {
      statusEl.textContent = '◉ ONLINE';
      statusEl.classList.add('puzzle-status--solved');
    } else if (unlocked) {
      statusEl.textContent = '◇ READY';
      statusEl.classList.add('puzzle-status--unlocked');
    } else {
      statusEl.textContent = '◌ STANDBY';
      statusEl.classList.add('puzzle-status--locked');
    }
  }

  // Show/hide answer display
  const answerEl = card.querySelector('.puzzle-card__answer');
  if (answerEl) {
    answerEl.style.display = solved ? '' : 'none';
  }

  // Enable/disable link on title
  const titleLink = card.querySelector('.puzzle-card__title a');
  if (titleLink) {
    if (unlocked || solved) {
      titleLink.removeAttribute('aria-disabled');
      titleLink.style.pointerEvents = '';
    } else {
      titleLink.setAttribute('aria-disabled', 'true');
      titleLink.style.pointerEvents = 'none';
    }
  }
}

/**
 * Refresh all puzzle cards on the page.
 */
function updateAllPuzzleCards() {
  const cards = document.querySelectorAll('.puzzle-card[data-puzzle-id]');
  cards.forEach(updatePuzzleCard);
}

/**
 * Update round progress labels.
 * Expects elements with class 'round-progress' and data-round attribute.
 */
function updateRoundProgress() {
  const progressEls = document.querySelectorAll('.round-section__progress[data-round]');
  progressEls.forEach(el => {
    const round = parseInt(el.dataset.round, 10);
    const { solved, total } = getRoundProgress(round);
    el.textContent = `${solved}/${total} SOLVED`;
  });
}

// ============================================================
// ANSWER SUBMISSION HANDLER
// ============================================================

/**
 * Attach answer submission behaviour to a form element.
 * Form must have: data-puzzle-id, .answer-input, .answer-result
 * @param {HTMLFormElement} form
 */
function attachAnswerHandler(form) {
  form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const puzzleId  = form.dataset.puzzleId;
    const inputEl   = form.querySelector('.answer-input');
    const resultEl  = form.querySelector('.answer-result');
    const submitBtn = form.querySelector('button[type="submit"]');

    if (!puzzleId || !inputEl) return;

    const userInput = inputEl.value.trim();
    if (!userInput) return;

    // Disable during async check
    if (submitBtn) submitBtn.disabled = true;
    inputEl.classList.remove('answer-input--correct', 'answer-input--incorrect');
    if (resultEl) {
      resultEl.className = 'answer-result';
      resultEl.textContent = '';
    }

    try {
      const result = await checkAnswer(puzzleId, userInput);

      if (result.correct) {
        inputEl.classList.add('answer-input--correct');
        if (resultEl) {
          resultEl.classList.add('answer-result--correct');
          resultEl.textContent = result.flavor || 'CORRECT. Log updated.';
        }
        // Refresh all card states — solving this puzzle may unlock others
        updateAllPuzzleCards();
        updateRoundProgress();
        // Check if FINAL was just solved
        if (puzzleId === 'FINAL') {
          triggerCommissionReveal();
        }
      } else {
        inputEl.classList.add('answer-input--incorrect');
        if (resultEl) {
          resultEl.classList.add('answer-result--incorrect');
          resultEl.textContent = 'INCORRECT. No match in log.';
        }
      }
    } catch (err) {
      console.error('Answer check failed:', err);
      if (resultEl) {
        resultEl.classList.add('answer-result--incorrect');
        resultEl.textContent = 'ERROR. Could not verify answer.';
      }
    } finally {
      if (submitBtn) submitBtn.disabled = false;
    }
  });
}

// ============================================================
// COMMISSION REVEAL
// ============================================================

/**
 * Show the commission overlay if FINAL is solved.
 */
function triggerCommissionReveal() {
  const overlay = document.querySelector('.commission-overlay');
  if (!overlay) return;
  overlay.classList.add('visible');
}

/**
 * Initialize commission overlay dismiss button.
 */
function initCommissionOverlay() {
  const overlay = document.querySelector('.commission-overlay');
  if (!overlay) return;

  const dismissBtn = overlay.querySelector('.commission-overlay__dismiss');
  if (dismissBtn) {
    dismissBtn.addEventListener('click', () => {
      overlay.classList.remove('visible');
    });
  }

  // Also show if already solved on page load
  if (isSolved('FINAL')) {
    overlay.classList.add('visible');
  }
}

// ============================================================
// PAGE INIT
// ============================================================

/**
 * Main initialization — run on DOMContentLoaded.
 */
function init() {
  // Update all puzzle cards to reflect stored state
  updateAllPuzzleCards();

  // Update round progress counters
  updateRoundProgress();

  // Attach answer forms
  const answerForms = document.querySelectorAll('form.answer-form[data-puzzle-id]');
  answerForms.forEach(attachAnswerHandler);

  // Commission overlay
  initCommissionOverlay();

  // Mark current nav link as active
  setActiveNav();
}

/**
 * Highlight the nav link matching the current page.
 */
function setActiveNav() {
  const currentFile = window.location.pathname.split('/').pop() || 'index.html';
  const navLinks = document.querySelectorAll('.site-nav a');
  navLinks.forEach(link => {
    const href = link.getAttribute('href') || '';
    if (href === currentFile || href.endsWith('/' + currentFile)) {
      link.classList.add('active');
    }
  });
}

// ============================================================
// ADMIN UTILITIES (used by admin.html)
// ============================================================

/**
 * Force-mark a puzzle as solved (admin override).
 * @param {string} puzzleId
 */
function adminMarkSolved(puzzleId) {
  markSolved(puzzleId);
  updateAllPuzzleCards();
  updateRoundProgress();
}

/**
 * Reset all solve state (admin utility).
 */
function adminResetAll() {
  try {
    localStorage.removeItem(STORAGE_KEY);
  } catch { /* silent */ }
  updateAllPuzzleCards();
  updateRoundProgress();
}

/**
 * Return a JSON string of current solve state (admin export).
 * @returns {string}
 */
function adminExportState() {
  const solved = [...getSolvedPuzzles()];
  return JSON.stringify({ solved, exported: new Date().toISOString() }, null, 2);
}

/**
 * Import solve state from a JSON string (admin import).
 * @param {string} jsonStr
 */
function adminImportState(jsonStr) {
  try {
    const data = JSON.parse(jsonStr);
    if (Array.isArray(data.solved)) {
      saveSolvedPuzzles(new Set(data.solved));
      updateAllPuzzleCards();
      updateRoundProgress();
    }
  } catch (err) {
    console.error('Import failed:', err);
  }
}

// ============================================================
// ENTRY POINT
// ============================================================

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
