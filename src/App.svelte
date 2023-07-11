<script lang="ts">
  import { createTabs } from "@melt-ui/svelte";
  import Decrypt from "./screens/Decrypt.svelte";
  import Encrypt from "./screens/Encrypt.svelte";
  import Identities from "./screens/Identities.svelte";

  const Tabs = {
    Identities: "Identities",
    Encrypt: "Encrypt",
    Decrypt: "Decrypt",
  };

  const { root, list, content, trigger } = createTabs({
    value: Tabs.Identities,
  });
</script>

<main class="p-5 pb-24 flex-1 flex flex-col">
  <div {...$root}>
    <div
      class="fixed bottom-0 inset-x-0 px-5 pb-5 from-transparent to-gray-100 bg-gradient-to-b"
    >
      <nav
        class="isolate flex divide-x divide-gray-200 rounded-lg shadow"
        {...$list}
      >
        <button {...$trigger(Tabs.Identities)} use:trigger class="trigger">
          Identities
          <span aria-hidden="true" class="trigger-indicator" />
        </button>

        <button {...$trigger(Tabs.Encrypt)} use:trigger class="trigger">
          Encrypt
          <span aria-hidden="true" class="trigger-indicator" />
        </button>

        <button {...$trigger(Tabs.Decrypt)} use:trigger class="trigger">
          Decrypt
          <span aria-hidden="true" class="trigger-indicator" />
        </button>
      </nav>
    </div>

    <div class="focus:outline-none" {...$content(Tabs.Identities)}>
      <Identities />
    </div>

    <div class="focus:outline-none" {...$content(Tabs.Encrypt)}>
      <Encrypt />
    </div>

    <div class="focus:outline-none" {...$content(Tabs.Decrypt)}>
      <Decrypt />
    </div>
  </div>
</main>
