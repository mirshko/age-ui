<script lang="ts">
  import IdentityForm from "./lib/IdentityForm.svelte";
  import { store } from "./lib/store";
  import { Tabs, TabList, TabPanel, Tab } from "./lib/tabs";
  import { dialog } from "@tauri-apps/api";

  function deleteIdentity(deleteIdentity: string) {
    return async () => {
      const confirmed = await dialog.confirm(
        "Are you sure you want to delete this identity?"
      );

      if (!confirmed) return;

      await store.delete(deleteIdentity);

      await store.save();
    };
  }
</script>

<main class="p-5">
  <Tabs>
    <TabList>
      <Tab>Identities</Tab>
      <Tab>Encrypt</Tab>
      <Tab>Decrypt</Tab>
    </TabList>

    <TabPanel>
      <IdentityForm />

      <div class="pt-4">
        {#await store.entries()}
          <p>...loading recipients</p>
        {:then keys}
          <ul class="space-y-4">
            {#each keys as [key, value]}
              <li>
                <div>
                  {value}
                </div>

                <button on:click={deleteIdentity(key)}>
                  Delete Identity
                </button>
              </li>
            {/each}
          </ul>
        {:catch error}
          <p style="color: red">{error.message}</p>
        {/await}
      </div>
    </TabPanel>

    <TabPanel>
      <div>
        <select>
          {#await store.values()}
            <option value="">Loading...</option>
          {:then keys}
            <option value="">Select</option>
            {#each keys as value}
              <option {value}>{value}</option>
            {/each}
          {/await}
        </select>
      </div>
    </TabPanel>

    <TabPanel>
      <h2>Third panel</h2>
    </TabPanel>
  </Tabs>
</main>
