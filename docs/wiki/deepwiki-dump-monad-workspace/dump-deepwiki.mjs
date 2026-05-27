#!/usr/bin/env node

import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { StreamableHTTPClientTransport } from "@modelcontextprotocol/sdk/client/streamableHttp.js";

const DEFAULT_REPO = "thomascarter613/monad-workspace";
const MCP_URL = "https://mcp.deepwiki.com/mcp";

const repoName = process.argv[2] || DEFAULT_REPO;
const outBase = process.argv[3] || path.join(process.cwd(), "docs", "deepwiki");

if (!repoName.includes("/")) {
  console.error(`Expected GitHub repo name in OWNER/REPO format. Received: ${repoName}`);
  process.exit(1);
}

const outDir = path.join(outBase, ...repoName.split("/"));

function jsonBlock(value) {
  return `\n\n\`\`\`json\n${JSON.stringify(value, null, 2)}\n\`\`\`\n`;
}

function toolResultToMarkdown(result) {
  const chunks = [];

  if (Array.isArray(result?.content)) {
    for (const item of result.content) {
      if (item?.type === "text" && typeof item.text === "string") {
        chunks.push(item.text);
      } else {
        chunks.push(jsonBlock(item));
      }
    }
  }

  if (result?.structuredContent) {
    chunks.push("## Structured Content" + jsonBlock(result.structuredContent));
  }

  if (chunks.length === 0) {
    chunks.push(jsonBlock(result));
  }

  return chunks.join("\n\n").trim() + "\n";
}

async function callDeepWikiTool(client, name, args) {
  console.log(`Calling ${name} for ${repoName}...`);

  const result = await client.callTool({
    name,
    arguments: args,
  });

  if (result?.isError) {
    throw new Error(`${name} returned an MCP tool error:\n${toolResultToMarkdown(result)}`);
  }

  return result;
}

async function main() {
  await mkdir(outDir, { recursive: true });

  const client = new Client({
    name: "deepwiki-dump-monad-workspace",
    version: "1.0.0",
  });

  const transport = new StreamableHTTPClientTransport(new URL(MCP_URL));

  try {
    console.log(`Connecting to ${MCP_URL}...`);
    await client.connect(transport);

    console.log("Listing available MCP tools...");
    const tools = await client.listTools();
    await writeFile(path.join(outDir, "tools.raw.json"), JSON.stringify(tools, null, 2));

    const structure = await callDeepWikiTool(client, "read_wiki_structure", { repoName });
    const contents = await callDeepWikiTool(client, "read_wiki_contents", { repoName });

    await writeFile(path.join(outDir, "structure.raw.json"), JSON.stringify(structure, null, 2));
    await writeFile(path.join(outDir, "contents.raw.json"), JSON.stringify(contents, null, 2));

    await writeFile(path.join(outDir, "structure.md"), toolResultToMarkdown(structure));
    await writeFile(path.join(outDir, "contents.md"), toolResultToMarkdown(contents));

    const readme = `# DeepWiki Dump: ${repoName}\n\nExported from DeepWiki MCP.\n\nSource MCP endpoint: ${MCP_URL}\n\n## Files\n\n- \`structure.md\` — DeepWiki documentation/topic structure.\n- \`contents.md\` — DeepWiki generated wiki contents.\n- \`structure.raw.json\` — raw MCP result for structure.\n- \`contents.raw.json\` — raw MCP result for contents.\n- \`tools.raw.json\` — available MCP tool metadata observed at export time.\n\n## Note\n\nThis is a dump of DeepWiki-generated documentation, not a clone of the GitHub source repository.\n`;

    await writeFile(path.join(outDir, "README.md"), readme);

    console.log(`\nDone. Wrote DeepWiki dump to:\n${outDir}`);
  } finally {
    try {
      await client.close();
    } catch {
      // Ignore close errors so a successful dump is not marked failed.
    }
  }
}

main().catch((error) => {
  console.error("\nDeepWiki dump failed.");
  console.error(error?.stack || error?.message || error);
  console.error("\nIf the error says the repository is not indexed, open the repo on DeepWiki first, wait for indexing, then rerun this script.");
  process.exit(1);
});
