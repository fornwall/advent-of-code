import gistMapping from "../gist-mapping.json";

const codeElement = document.getElementById("code");
const compilerExplorerLinkElement = document.getElementById(
  "compilerExplorerLink"
);
const assemblyElement = document.getElementById("assembly");

const state = {
  params: {},
};

async function onLoad() {
  const hash = location.hash.substring(1);
  for (const part of hash.split("&")) {
    const [key, value] = part.split("=");
    state.params[key] = decodeURIComponent(value);
  }

  const gistDayInfo = gistMapping[state.params.year][state.params.day];
  const daySource = await (await fetch(gistDayInfo["raw_url"])).text();
  codeElement.textContent = daySource;
  Prism.highlightElement(codeElement);

  const assemblyResponse = await fetch(
    "https://godbolt.org/api/compiler/beta/compile?options=" +
      encodeURIComponent("-C opt-level=2"),
    {
      method: "post",
      body: daySource,
    }
  );
  const assemblyText = await assemblyResponse.text();
  assemblyElement.textContent = assemblyText;
  Prism.highlightElement(assemblyElement);

  const clientState = {
    sessions: [
      {
        id: 1,
        language: "rust",
        source: daySource,
      },
    ],
  };
  const clientStateJson = JSON.stringify(clientState);
  const clientStateBase64 = btoa(clientStateJson);
  compilerExplorerLinkElement.href =
    "https://godbolt.org/clientstate/" + encodeURIComponent(clientStateBase64);
}

document.addEventListener("DOMContentLoaded", onLoad);
