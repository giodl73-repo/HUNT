# /hunt script — Generate a Game Architecture Script

Produce a complete game director's blueprint for any puzzle hunt. Turns puzzle specs, world data, and narrative design into a scene-by-scene game script — the document a game developer uses to build the full experience.

## Usage

```
/hunt script              — generate GAME-SCRIPT.md for the current scenario
/hunt script phaser       — Phaser 4/TypeScript output (scenes, registry, GameObjects)
/hunt script react        — React/TypeScript output (components, state, hooks)
/hunt script unity        — Unity C# output (MonoBehaviours, SceneManager)
/hunt script web          — Static HTML/CSS/JS output (for browser-based hunts)
/hunt script update       — regenerate after puzzle changes
```

---

## What It Produces

`delivery/GAME-SCRIPT.md` — a single document covering:

1. **World Architecture** — map all locations (rooms, decks, ships) with puzzle-to-room assignments, navigation connections, and access states
2. **Navigation System** — how the player moves (turbolifts, corridors, doors), door state machine (OPEN / LOCKED / SECURED / FORCED), unlock trigger table
3. **Full Scene Script** — every scene in the game, in order, with: location, trigger, display content, audio, duration, what unlocks next
4. **State Machine** — complete game state interface with all enums, transitions, unlock logic, answer validation, save/load contract, widget map
5. **Multi-Ship / Future Missions** — how this mission fits the broader game world, career progression gates, implementation priority order

---

## Section 1: World Architecture

Read the scenario's world files and puzzle files. Map every puzzle to a physical location:

```
For each puzzle:
  → which department/room does it belong to?
  → what is the room's deck/floor?
  → what are the room ID and name?
  → what widget(s) does the puzzle use?
  → what does the player see when they enter?
```

Output a complete room map organized by deck/floor. Include:
- Room ID (format varies by world — `{deck}-{section}-{room}` for cruisers, panel locations for shuttlepods)
- Department (matches world/ department system)
- Puzzle assigned (if any)
- Widget list
- Navigation connections (what doors lead where)

---

## Section 2: Navigation System

Define how the player moves through the world:

**Navigation primitives** (choose based on game framework):
- Turbolift / elevator (vertical between decks)
- Corridor (horizontal within a deck)
- Door states: OPEN (accessible), LOCKED (puzzle gate), SECURED (story gate), FORCED (evidence of tampering — visual only)

**Navigation state machine:**

| Phase | Accessible areas | Locked areas | Unlock trigger |
|-------|-----------------|--------------|----------------|
| Opening | [starting area] | [everything else] | [intro complete] |
| Round 1 active | [Round 1 wing] | [Rounds 2, 3] | [all R1 puzzles solved] |
| ... | ... | ... | ... |

For each locked area, specify:
- What the player sees when they try to enter (message, visual)
- Exactly what unlocks it (puzzle ID, event, cutscene complete)

---

## Section 3: Full Scene Script

Write every scene in the game. Use this format:

```
SCENE [ID]: [TITLE]
LOCATION: [Room name, Deck X]
TRIGGER: [what causes this scene to play]
DISPLAY: [what the player sees on screen — be specific]
AUDIO: [ambient loop, music cue, sound effect]
DURATION: [auto-advance after N seconds, or player-triggered]
LOG ENTRY: [if this scene delivers a ship log, the exact text in terse format]
UNLOCK: [what becomes accessible after this scene completes]
```

**Scene types to include:**

- **Opening sequence** — how the player enters the world, what establishes the stakes, why this matters
- **Puzzle unlock scenes** — brief beat when a new room/console becomes available
- **Puzzle completion scenes** — the log entry or data readout that appears when a puzzle is solved (terse, evidence-only, no exposition)
- **Narrative turn scenes** — the moments where the story shifts. These are the most important scenes. Give them weight: longer duration, no continue button, specific visual treatment
- **Round complete scenes** — transition between rounds, what the player now knows, what unlocks
- **Meta solve scenes** — when a round meta is solved, what does the player learn?
- **The reveal** — the final answer. What does the screen look like? How long does it hold? What happens after?
- **The decision** — does the player have a choice? What are the options? What do the outcomes look like?
- **Epilogue** — the ending. Both/all paths.

**Key principle:** Narrative turns must earn their weight. If the story has a "the ship was prepared before contact" moment, that scene should:
- Appear full-screen
- Display the evidence in terse data format
- Hold without a continue button for 3-5 seconds
- Have no explanation — the player feels it themselves

---

## Section 4: State Machine

Output the complete game state for the target framework.

### Phaser 4 output (`/hunt script phaser`)

```typescript
// State stored in Phaser Registry (this.registry)
// Access from any Scene: this.registry.get('gameState')

interface GameState {
  phase: GamePhase;
  currentScene: string;
  solvedPuzzles: Record<string, boolean>;
  unlockedRooms: Record<string, RoomState>;
  metaValues: { r1: number|null, r2: number|null, r3: number|null };
  finalAnswer: string|null;
  countdown: number;        // seconds remaining
  decision: DecisionChoice|null;
}

// Scene transitions
this.scene.start('SceneKey');          // full transition
this.scene.sleep('SceneKey');          // pause, keep in memory
this.scene.wake('SceneKey');           // resume paused scene
this.scene.launch('OverlayScene');     // run in parallel (for cutscenes)

// Puzzle completion event
this.events.emit('puzzle:solved', puzzleId);
// Scene Manager listens → updates registry → triggers unlock + cutscene queue

// Timed text reveal (e.g., badge log appearing line by line)
this.tweens.createTimeline({ ... });
// or: this.time.delayedCall(2000, () => showNextLine());
```

Include the complete widget map: for each widget class, the source file (`ConsoleWidgets.ts` / `InteractiveControls.ts`), the Scene it appears in, and how it's instantiated.

### React/TypeScript output (`/hunt script react`)
Use React context + useReducer for global state. Scene transitions via React Router. Cutscene sequences via setTimeout chains or Framer Motion.

### Static web output (`/hunt script web`)
Use localStorage for state persistence. Page-per-puzzle with JavaScript unlock logic. CSS transitions for cutscenes.

---

## Section 5: Multi-World / Future Missions

If the world supports multiple ship types, locations, or career progression:

- Map how this scenario fits within the larger game world
- Define what the player earns by completing this mission (rank, access, unlock)
- Define what missions become available next
- Define the career/progression gate system
- List shared infrastructure (shared widget library, shared world data, shared state format)
- Suggest implementation priority for expanding the world

---

## Progressive Unlock Design

If the game uses progressive unlock (start simple, earn more space):

Specify the exact unlock sequence:
1. Player starts in [smallest accessible space]
2. First solve unlocks [next room/console]
3. Completing [set] unlocks [next section]
4. Full access earned at [milestone]

The principle: the world reveals itself as the player earns it. Never show all rooms at once.

---

## Implementation Notes

After writing the script, append a section for the developer:

- **Build order** — what to implement first (state machine → navigation → simplest puzzle → cutscenes → all puzzles → meta → reveal → epilogue)
- **Testing milestones** — what constitutes a shippable vertical slice at each stage
- **Widget inventory** — complete list of widgets needed with source files
- **Audio design** — ambient loops per location, any recurring motifs, key sound moments
- **Accessibility** — colorblind-safe color choices, text size, keyboard navigation if applicable

---

## Backport Reminder

After writing this skill, copy it to `~/.claude/skills/hunt-script/SKILL.md` in the reference library.
