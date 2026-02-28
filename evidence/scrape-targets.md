# Puzzle Scraping Target List
*Run agents with mode: "bypassPermissions" to execute bash scripts*

---

## TIER 1 — Fully Public Archives (highest yield)

### MIT Mystery Hunt Archives (puzzles.mit.edu)
All pre-2016 hunts are fully public. URL pattern: `https://puzzles.mit.edu/{year}/puzzle/{slug}`

| Year | Theme | Authors to target | # Puzzles |
|------|-------|-------------------|-----------|
| 2000 | Tossed Salad | Dan Katz (8 puzzles incl. Duck Konundrum) | ~80 |
| 2002 | The Biggest Little Hunt | Dan Katz (13 puzzles incl. D2 Duck Konundrum) | ~100 |
| 2005 | Normalville | Dan Katz (28 puzzles incl. D3 Duck Konundrum) | ~105 |
| 2007 | The Apprentice | Mike Selinker (22), Mark Gottlieb (10), Dan Katz (38) | ~150 |
| 2009 | Zyzzlvaria | Ian Tullis (27), Mike Selinker (44), Mark Gottlieb (6), Dan Katz (33) | ~200 |
| 2012 | MIT: The Musical | Wei-Hwa Huang (Equal Billing) | ~150 |
| 2019 | A Hundred Years of Mystery | Dan Katz (19 puzzles) — won with Left Out | ~175 |
| 2020 | Penny Park | Ian Tullis (31), Wei-Hwa Huang (14), Larry Hosken (7), Dan Egnor (1) | ~170 |
| 2025 | — | Dan Katz (On the Corner) | ~200 |

**Bash script:**
```bash
# Crawl MIT MH 2009 puzzle list
curl -s "https://puzzles.mit.edu/2009/" | grep -o 'href="/2009/puzzle/[^"]*"' | sort -u > evidence/mit2009-urls.txt
while read url; do
  slug=$(echo $url | sed 's/href="\/2009\/puzzle\///;s/"//')
  curl -s "https://puzzles.mit.edu/2009/puzzle/$slug" >> "evidence/mit2009-puzzles/$slug.html"
  sleep 0.5
done < evidence/mit2009-urls.txt
```

### P&A Magazine Sample
- URL: `https://www.pandamagazine.com/panda_sampler.pdf`
- Contains Foggy Brume's style intro puzzles

### Puzzle Boat (public pages)
- `https://www.pandamagazine.com/island{N}/` for N=2..12

### Grandmaster Puzzles (Thomas Snyder)
- `https://www.gmpuzzles.com/blog/` — hundreds of specific Snyder puzzle posts
- Each post contains a specific puzzle type (Sudoku variant) with mechanics described

### Puzzlvaria (Dan Katz)
- `https://puzzlvaria.wordpress.com/` — full archive of hunt reviews
- Every post contains named concepts, specific puzzle analysis

---

## TIER 2 — Partially Accessible

### puzzle.university (MS PH 23)
- Round index pages: PUBLIC (`/round/*.html`)
- Individual puzzle pages: PUBLIC (`/puzzle/*.html`)
- Solution pages: PUBLIC (`/solution/*.html`)
- **Already scraped**: Full Economics + Placement Test rounds
- **Still needed**: Music, English, History, CS (full), Math, Classics, Sociology, Interludes rounds

### puzzlehunt.azurewebsites.net (MS PH 21/22/24)
- Home pages: PUBLIC
- Sample puzzles: PUBLIC (`/ph{N}/play/samples`)
- Puzzle lists: **REQUIRES LOGIN**
- **User has admin access** — can export puzzle list + content

---

## TIER 3 — Requires Special Access

### MS Puzzlehunt pre-PH23 puzzle archives
- PH 20 (Theater): `puzzlehunt.azurewebsites.net/ph20/play/Puzzles` — LOGIN REQUIRED
- PH 21 (Video Games): `puzzlehunt.azurewebsites.net/ph21/play/Puzzles` — LOGIN REQUIRED
- PH 22 (Parallax): `puzzlehunt.azurewebsites.net/ph22/play/Puzzles` — LOGIN REQUIRED

### Galactic Puzzle Hunt Archive
- `https://galacticpuzzlehunt.com/` — multiple years public

### MIT MH 2020 Full Puzzle List
- `https://puzzles.mit.edu/2020/puzzles` — may need correct URL format

---

## Agent Script Template (bypassPermissions)

```javascript
// Run as agent with mode: "bypassPermissions"
// Scrapes MIT MH puzzle archive and saves to evidence/

const urls = [
  "https://puzzles.mit.edu/2009/puzzles/",
  "https://puzzles.mit.edu/2007/puzzles.html",
  "https://puzzles.mit.edu/2005/puzzles.html",
];

// For each URL, fetch + extract puzzle titles, authors, mechanics
// Save to evidence/mit-mh-{year}-puzzles.md
```

---

## Priority Order for Scraping

1. **puzzle.university remaining rounds** — Music, English, History, CS, Math, Classics, Sociology (public, no auth needed)
2. **MIT MH 2009** — biggest single-year haul for Selinker/Tullis/Katz/Gottlieb
3. **MIT MH 2007** — Selinker 22 puzzles + Gottlieb 10 + Katz 38
4. **MIT MH 2000/2002/2005** — Dan Katz + Duck Konundrum original
5. **MS PH 21/22/24** — user logs in and exports puzzle lists
6. **Grandmaster Puzzles blog** — Thomas Snyder specific puzzle types
