<script lang="ts">
  import { onMount, type Snippet } from 'svelte';

  import '../app.css';

  import { goto } from '$app/navigation';
  import { db } from '$lib/instant';

  type Props = {
    children: Snippet;
  };

  let { children }: Props = $props();

  onMount(() => {
    const unsub = db.subscribeAuth((auth) => {
      if (!auth.user) {
        goto('/login');
      }
    });

    return unsub;
  });
</script>

{@render children()}
