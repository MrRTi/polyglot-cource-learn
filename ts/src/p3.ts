function getInput(): string {
  return `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`;
}

enum SlopeElmt {
  Tree,
  Snow,
}

const slopeElmts = getInput()
  .split("\n")
  .map((x) =>
    x.split("").map((x) => (x === "." ? SlopeElmt.Snow : SlopeElmt.Tree)),
  );

const colLen = slopeElmts[0].length;
let treeCount = 0;

slopeElmts.forEach((slopeRow, i) => {
  if (slopeRow[(i * 3) % colLen] === SlopeElmt.Tree) {
    treeCount++;
  }
});

console.log("Tree count", treeCount);
