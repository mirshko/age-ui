<script lang="ts">
  import { createTabs } from "@melt-ui/svelte";
  import Decrypt from "./screens/Decrypt.svelte";
  import Encrypt from "./screens/Encrypt.svelte";
  import Identities from "./screens/Identities.svelte";

  const { root, list, content, trigger } = createTabs({
    value: "Identities",
  });
</script>

<main class="p-5">
  <div {...$root}>
    <div
      class="fixed bottom-0 inset-x-0 px-5 pb-5 from-transparent to-gray-100 bg-gradient-to-b"
    >
      <nav
        class="isolate flex divide-x divide-gray-200 rounded-lg shadow"
        {...$list}
      >
        <button {...$trigger("Identities")} use:trigger class="trigger">
          Identities
          <span aria-hidden="true" class="trigger-indicator" />
        </button>

        <button {...$trigger("Encrypt")} use:trigger class="trigger">
          Encrypt
          <span aria-hidden="true" class="trigger-indicator" />
        </button>

        <button {...$trigger("Decrypt")} use:trigger class="trigger">
          Decrypt
          <span aria-hidden="true" class="trigger-indicator" />
        </button>
      </nav>
    </div>

    <div class="focus:outline-none" {...$content("Identities")}>
      <Identities />
    </div>

    <div class="focus:outline-none" {...$content("Encrypt")}>
      <Encrypt />
    </div>

    <div class="focus:outline-none" {...$content("Decrypt")}>
      <Decrypt />
    </div>
  </div>
</main>
