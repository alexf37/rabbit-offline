console.log("Starting...");
const args = Bun.argv;

const url = args[2];
let depth = Number(args[3]);
if (depth === 0 || depth === undefined || isNaN(depth)) {
  depth = 1;
}

async function downloadPageRecursive(
  url: string,
  filePath: string,
  currentDepth: number
) {
  if (currentDepth > depth) {
    return;
  }
  if (await Bun.file(filePath).exists()) {
    console.log("file already exists:", filePath);
    return;
  }
  const response = await fetch(url);
  const content = await response.text();

  await Bun.write(filePath, content);
}
