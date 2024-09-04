import { Elysia } from "elysia";

const app = new Elysia();

app.get("/*", async (req) => {
  let filePath = req.path;

  if (filePath.endsWith("/")) {
    filePath += "index";
  }

  const file = Bun.file(`.${filePath}`);
  req.set.headers["content-type"] = "text/html";

  // check if file exists and does not have .html extension
  const fileExists = await file.exists();
  console.log(fileExists);

  if (fileExists) {
    return await file.text();
  } else {
    return `File not found: ${file.name}`;
  }
});

app.listen(3000, () => console.log(`Server running on http://localhost:3000/`));
