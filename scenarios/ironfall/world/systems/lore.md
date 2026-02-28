# IRONFALL Lore & Characters

Source of truth for playable characters, NPCs, story quests, and dialogue.

---

## Playable Characters (4)

### Rynn — The Wanderer
- **Role:** Warrior / Fire specialist
- **Age:** 19
- **Weapon:** Greatsword
- **Backstory:** Grew up in Ashveil Village. Left home after the Iron King's army burned the forest's northern edge. Seeks revenge but finds something else along the way.
- **Backstory Quest:** "The Burning Road" — return to the site of the fire, find Rynn's mother's sword buried in the ash.
- **Key dialogue:** "The fire took everything. But it also cleared the path."
- **Special Moves:** Blazing Arc, Pyre Storm

### Elara — The Scholar
- **Role:** Healer / Ice specialist
- **Age:** 24
- **Weapon:** Staff
- **Backstory:** Former student at the Frosthold Academy, expelled for researching forbidden ice magic. Travels to prove her theories about the connection between ice and memory.
- **Backstory Quest:** "Frozen Memories" — find the 3 Memory Crystals in Glacial Reach to reconstruct the lost spell.
- **Key dialogue:** "The ice remembers. That is both its gift and its curse."
- **Special Moves:** Frost Needle, Absolute Zero

### Kael — The Guardian
- **Role:** Tank / Thunder specialist
- **Age:** 32
- **Weapon:** Spear and shield
- **Backstory:** Former captain of the Stormspire garrison. Deserted after discovering the Storm Tyrant was once his commanding officer, transformed by dark magic.
- **Backstory Quest:** "The Broken Oath" — climb Storm Tower to face the Storm Tyrant and learn the truth about the garrison's fall.
- **Key dialogue:** "I swore to protect this mountain. The oath did not say from whom."
- **Special Moves:** Thunder Lance, Storm Judgment

### Maren — The Diver
- **Role:** Rogue / Dark specialist
- **Age:** 27
- **Weapon:** Twin daggers
- **Backstory:** Treasure hunter from Tidepool Village. Has been diving the Drowned Temple since childhood, searching for the Abyssal Crown — an artifact that supposedly grants passage to the world beneath the sea.
- **Backstory Quest:** "The Deep Call" — reach the lowest level of the Drowned Temple, find the Abyssal Crown, and choose whether to use it.
- **Key dialogue:** "Down is just another direction. You get used to the dark."
- **Special Moves:** Shadow Rend, Void Collapse

---

## NPCs (6)

### Ashveil NPCs

**Old Forge — Blacksmith**
- Location: Ashveil Village
- Role: Teaches crafting, operates the Forge
- Key dialogue: "Bring me two things and I will make them one. But choose carefully — not all combinations are kind."
- Gives: Forge Key (after first dungeon)

**Tome — Lore Keeper**
- Location: Ashveil Village library
- Role: Explains the Marks of Mastery system
- Key dialogue: "The Marks are not trophies. They are mirrors. Each one shows you what you have become."
- Gives: Mark of Seedling (tutorial completion)

### Region NPCs

**Garek — The Ferryman**
- Location: Between Ashveil Forest and Glacial Reach
- Role: Controls passage north
- Key dialogue: "The forest key opens the gate. But the gate is not the only way. Some paths are found by those who are not looking."
- Secret: If you talk to Garek 7 times without having the Forest Key, he gives a hint about the Ember Wastes shortcut.

**Sable — Shadow Merchant**
- Location: Appears in any town after 10 PM game time
- Role: Sells rare items at high prices, buys rare drops
- Key dialogue: "Everything has a price. Some prices are paid in coin. Others in time. Others in choices."
- Secret: Sable's inventory changes based on how many Marks of Mastery you have. At 12+ Marks, she sells the Void Essence.

### Iron Citadel NPCs

**Morimoto's Ghost**
- Location: Iron Gate (save point room)
- Role: Developer self-insert. Speaks in fourth-wall-breaking riddles.
- Dialogue (varies by game state):
  - First visit: "You reached the gate. Most players stop here. You are not most players."
  - With Developer's Key: "You found my key. Now find my room. The answer is below, not above."
  - After defeating Shadow: "The code is the last thing I wrote. It is scattered in the things I built. Look at what the game remembers."
  - All Marks + Developer's Key: "The Marks. The enemies. The items. The moves. The rooms. I hid a letter in each system. Collect them all and you have the code."

**The Iron King (pre-boss dialogue)**
- Location: Throne Room, Citadel 7F
- Dialogue: "You came seeking an ending. I am the ending. Unless you know another way."

---

## The True Ending

### Canonical Path
1. Complete the game once (defeat the Iron King) → Standard ending plays
2. On the save file, all standard Marks earned
3. The Developer's Key and Astral Compass must be in inventory
4. On the title screen (after "The End"), enter the cheat code
5. A new cutscene plays: Morimoto's ghost appears one last time, the Iron Citadel transforms into a garden, and the words appear: "Thank you for playing. — K.M."

### The Cheat Code
The True Ending cheat code is a 12-button sequence on the SNES controller, entered on the title screen. The code maps to game data — each button press corresponds to a specific discovery across the game's systems. The super-meta of the puzzle hunt reconstructs this code.

### Forum Theories (in-universe, for puzzle framing)
- **User DarkKnight_97:** "I've been trying button combos on the title screen for 3 years. Nothing works. The debug menu rumor was debunked."
- **User IronFan_Kenji:** "Morimoto said 'look at what the game remembers.' I think the achievements are part of it."
- **User SpeedQueen:** "I ran the game through a hex editor. There's a subroutine that checks for a 12-input sequence at the title screen. It's real."
- **User LoreHunter:** "The lore tablets are numbered 1-12. The cheat code is 12 buttons. Coincidence?"
- **User DataMiner_X:** "I found partial bytecode: the first input is checked against address 0x7F03A2. That's in the bestiary data block."

---

## NPC Dialogue Cipher

Several NPCs use a subtle cipher in their dialogue. Certain words are capitalized that should not be, or certain sentences have unusual first letters. This is a cipher left by the developer.

Examples from Morimoto's Ghost dialogue:
- "**U**nless you know another way" → U
- "**P**layers stop here" → P

The cipher is: take the first letter of each sentence Morimoto speaks across ALL his appearances. This spells a message. (The full message is part of the meta design and stored in encoded form only.)

---

## Story Timeline

| Year (in-game) | Event |
|----------------|-------|
| Ancient | The Old Gods create the 6 elements and the continent |
| 200 years ago | The Iron King rises, builds the Citadel |
| 100 years ago | The Iron King is sealed but not destroyed |
| 30 years ago | Ashfield Interactive founded (real-world: 1963 → in-game lore) |
| Present day | The Iron King awakens. Rynn's adventure begins. |
| Post-game | Morimoto's ghost appears. The True Ending is possible. |
