# Layout Spec: P4 -- The Witness Statement

**Exhibit:** D
**Document type:** Typed police interview transcript
**Source:** `puzzles/P4-witness-statement.md`
**Print-ready:** `puzzles/P4-witness-statement.html` (to be created from source)
**Pages:** 2 (stapled)
**Stock:** White 20lb letter paper

---

## Document Identity

This is a typed transcript of a police interview: typewriter font, speaker labels in bold, stage directions in italics within brackets, indented paragraphs. It reads like a document produced by a police stenographer -- transcribed verbatim, with the witness's verbal tics, pauses, and corrections faithfully recorded.

The visual style matches P1 (same police department header, same Courier New body font) but the content format is distinctly a transcript: dialogue structure, speaker identifiers, and bracketed annotations.

---

## Page 1 Layout

### Header Box (top of page)
- Same police department header box as P1 and the evidence log:
  - "CITY POLICE DEPARTMENT" -- Georgia 14pt bold, centered
  - "WITNESS STATEMENT" -- Georgia 12pt regular, centered
  - Case No., Date, Interviewing Officer -- Courier New 9pt
  - Witness: Miss Colette Delacroix, Room 205
  - Location: The Grand Hotel, Manager's Office (Room 106)
  - Time of Interview: 11:15 PM, October 11, 1947
  - Note: "Statement transcribed verbatim. Witness was cooperative but visibly agitated. Occasional pauses noted with [pause]."
- Margin below header: 20px

### Opening exchange
- **Det. Callahan:** -- bold, Courier New 10pt
- Question text in regular weight
- Followed by:
- **Miss Delacroix:** -- bold, Courier New 10pt

### Main testimony (the acrostic text)
The bulk of the document: Miss Delacroix's continuous statement. This is where the 32 sentences live, with their first letters spelling the hidden message.

**Formatting:**
- Continuous prose paragraphs (not numbered sentences)
- Indented paragraph style: first line of each new paragraph indented 0.5 inches
- No bullet points, no numbering -- this is transcribed speech
- Stage directions: `[pause]` in italic, on its own line or inline
- Line height: 1.5 (generous, to give the text room to breathe and make the first-letter pattern subtly scannable)

**Paragraph breaks in the testimony:**
The 32 sentences are organized into natural paragraphs. Based on the source:

Paragraph 1 (sentences 1-4): Arriving at the gala, going to the gallery, examining the painting.
Paragraph 2 (sentences 5-7): The attendant, Mrs. Ashworth passing through.
Paragraph 3 (sentences 8-12): Feeling unwell, deciding to leave the gallery at 9:00.
Paragraph 4 (sentences 13-18): Standing in the hallway, looking out the window, the quiet of the upper floors.
[pause -- stage direction]
Paragraph 5 (sentences 19-24): Hearing a sound (the elevator), walking back toward her room.
Paragraph 6 (sentences 25-28): Passing the gallery, seeing the empty chair, not going in.
Paragraph 7 (sentences 29-32): The room appeared normal, curtains moving, not suspecting anything.
[pause -- stage direction]
Final paragraph: Going to her room, hearing the commotion, learning the painting was gone.

### Page break
After the main testimony block, before the follow-up question.

---

## Page 2 Layout

### Follow-up exchange
- **Det. Callahan:** -- bold, follow-up question about other people on the second floor
- **Miss Delacroix:** -- bold, response about seeing a shadow near the staircase (tall figure, dark suit, moved toward 3F)
- This exchange corroborates the Fontaine sighting and provides cross-puzzle interlock with P1

### End of statement
- Italic note: "End of statement. Witness signed and dated."
- Horizontal rule

### Section: INVESTIGATOR'S ANALYSIS (worksheet)
- Bordered box: 1px solid #1a1a1a, light background (#fafafa)
- Instruction text: "Read Miss Delacroix's testimony carefully. Her words reveal more than she intended."
- "Take the first letter of each sentence in her main statement..."
- Sentence enumeration grid:

```
Sentence  1: "I came to the gala..."                           -> ___
Sentence  2: "Studying art is my life..."                       -> ___
...
Sentence 32: "The thought of anything being wrong..."           -> ___
```

- 32 rows, each with sentence fragment and a blank for the first letter
- Courier New 9pt for the grid (compact to fit on page)

- Instruction: "Write all 32 first letters: ________________________________"
- "Read the hidden message. The last SIX letters of the message spell the answer."
- Answer grid: 6 cells, 28px square, centered
- Label: "(6 letters)"

---

## Transcript Formatting Details

### Speaker labels
- Bold: **Det. Callahan:** and **Miss Delacroix:**
- Followed by regular-weight text
- Speaker labels at the left margin; speech text after the colon

### Stage directions
- Format: `[pause]` -- italic, within square brackets
- Can appear inline ("I could not believe it. [pause] I wish I had...") or on their own line between paragraphs
- Font: Courier New italic, same size as body text

### Voice characteristics (reflected in formatting)
- Em dashes for mid-sentence breaks: "the attendant -- Mr. Gould -- was seated..."
- Verbal hedges preserved: "I mean", "you understand", "well"
- Self-corrections: sentences that start one way and redirect
- These are part of the text content, not formatting choices, but the layout should preserve them faithfully

### Line spacing
- Body text: 1.5 line height (generous)
- This spacing is important: it makes each sentence's first letter slightly more scannable without being obvious. At 1.5 spacing, the left margin has a rhythm that rewards careful attention.

---

## The Acrostic

The first letters of the 32 sentences spell:

```
I-S-A-W-T-H-E-C-A-N-V-A-S-I-N-S-I-D-E-A-W-O-O-D-E-N-O-B-J-E-C-T
```

Which reads: "I SAW THE CANVAS INSIDE A WOODEN OBJECT"

The last 6 letters: O-B-J-E-C-T = the answer word.

The acrostic is designed to be invisible on a casual read (the sentences sound natural as spoken testimony) but discoverable once the solver thinks to check first letters. The analysis section guides them to this check.

---

## Voice Notes

- The testimony is in Delacroix's voice: earnest, nervous, verbose, full of pauses
- The detective's questions are minimal -- one at the start, one follow-up
- The police header and analysis section are in institutional police voice
- The contrast between Delacroix's emotional testimony and the clinical worksheet below it reinforces the noir tone

---

## Print Notes

- Page 1 should hold the header and the full main testimony. The testimony is ~32 sentences across ~7 paragraphs. At 10pt Courier New with 1.5 line height, this fits on one page (barely) with 0.6in top/bottom margins.
- If tight, reduce line height to 1.4 or font to 9.5pt -- but preserve readability
- Page 2 holds the follow-up exchange, end-of-statement note, and the analysis worksheet
- The sentence enumeration grid (32 rows) is the densest part. At 9pt with minimal spacing, it fits in approximately 4.5 inches of vertical space.
- Staple the two pages together at the top-left corner
