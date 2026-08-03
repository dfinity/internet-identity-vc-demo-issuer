/* Vite helpers used by `vite.config.ts`.
 *
 * Vendored from `dfinity/internet-identity`, `src/vite-plugins`, reduced to the
 * two helpers the issuer frontend needs so that this repository does not depend
 * on II's npm workspace. */

import { execSync } from "child_process";
import type { Plugin } from "vite";

/** Reads a canister's id from the local icp-cli network. */
export const readCanisterId = ({
  canisterName,
}: {
  canisterName: string;
}): string => {
  const command = `icp canister status ${canisterName} --id-only`;
  try {
    const stdout = execSync(command);
    return stdout.toString().trim();
  } catch (e) {
    throw Error(
      `Could not get canister ID for '${canisterName}' with command '${command}', was the canister deployed? ${e}`,
    );
  }
};

/** The origin of the local replica's HTTP gateway. */
export const getReplicaHost = (): string => {
  try {
    const stdout = execSync("icp network status --json");
    const status = JSON.parse(stdout.toString());
    const port = new URL(status.gateway_url).port;
    return `http://127.0.0.1:${port}`;
  } catch (e) {
    throw Error(`Could not get replica port, is the replica running? ${e}`);
  }
};

/**
 * Inject the canister ID of 'canisterName' as a <script /> tag in index.html for local development. Will process
 * at most 1 script tag.
 */
export const injectCanisterIdPlugin = ({
  canisterName,
}: {
  canisterName: string;
}): Plugin => ({
  name: "inject-canister-id",
  transformIndexHtml(html): string {
    const rgx = /<script type="module" src="(?<src>[^"]+)"><\/script>/;
    const canisterId = readCanisterId({ canisterName });
    return html.replace(rgx, (_match, src) => {
      return `<script data-canister-id="${canisterId}" type="module" src="${src}"></script>`;
    });
  },
});
