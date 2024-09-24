<script lang="ts">
  import { goto } from '$app/navigation';
  import { db } from '$lib/instant';
  import { type InstantQueryResult, type User, id } from '@instantdb/core';
  import { onMount } from 'svelte';

  type Todos = InstantQueryResult<typeof db, { todos: {} }>['todos'];

  // let { teamName, teamId }: { teamName: string; teamId: string } = ;

  let user: User | undefined = $state();
  let teamName = $state('');
  let teamId = $state('');
  let todos: Todos = $state([]);
  let todoText: string = $state('');
  let todoDate: number | undefined = $state();

  // export function load({
  //   params,
  // }: {
  //   params: { teamName: string; teamId: string };
  // }) {
  //   console.log('SUPER HERE');
  //   console.log(params);
  //   teamName = params.teamName;
  //   teamId = params.teamId;
  // }

  onMount(() => {
    const unsub = db.subscribeAuth((auth) => {
      user = auth.user;
    });

    const unsubQuery = db.subscribeQuery(
      { todos: { $: { where: { teams: teamId } } } },
      (resp) => {
        if (resp.data) {
          todos = resp.data.todos;
        }
      },
    );

    return () => {
      unsub();
      unsubQuery();
    };
  });

  async function createTodo(e: Event) {
    e.preventDefault();

    console.log(teamId);
    console.log(teamName);

    if (user) {
      try {
        const todoId = id();

        const result = await db.transact([
          db.tx.todos[todoId].update({
            text: todoText,
            done: false,
            date: todoDate || Date.now(),
          }),
        ]);

        console.log(result);
      } catch (e) {
        console.log(e);
      }
    }
  }
</script>

<div>
  <h1>TODOs for Team{teamName}</h1>

  <button
    type="button"
    onclick={() => {
      goto('/');
    }}>Back</button
  >

  <form onsubmit={createTodo} class="flex flex-col gap-8">
    <input
      type="text"
      class="border rounded p-2 focus:border-red-400 focus:outline-none focus:border-2 shadow-inner"
      bind:value={todoText}
      placeholder="Write a Todo"
    />

    <input
      type="text"
      class="border rounded p-2 focus:border-red-400 focus:outline-none focus:border-2 shadow-inner"
      bind:value={todoDate}
      placeholder="Todo Due Date"
    />

    <button
      type="submit"
      class="bg-purple-700 text-cyan-100 p-2 rounded-lg shadow-md hover:bg-purple-600 transition-transform transform-gpu hover:translate-y-[-2px]"
      >Create</button
    >
  </form>

  {#each todos as todo}
    <div class="p-8 flex flex-col gap-4">
      <p>{todo.id},{todo.text},{todo.date},{todo.done}</p>
    </div>
  {/each}
</div>
