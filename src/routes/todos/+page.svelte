<script lang="ts">
  import { goto } from '$app/navigation';
  import { db } from '$lib/instant';
  import { teamState } from '$lib/team.svelte';
  import { type InstantQueryResult, type User, id } from '@instantdb/core';
  import { onMount } from 'svelte';

  type Todos = InstantQueryResult<typeof db, { todos: {} }>['todos'];
  type Todo = Todos[0];

  const selectedTeamState = teamState();

  // empty array for each day of the week
  let todos: [Todos, Todos, Todos, Todos, Todos, Todos, Todos] = $state([
    [],
    [],
    [],
    [],
    [],
    [],
    [],
  ]);
  let todoText: string = $state('');
  let todoDate: number | undefined = $state();
  const weekStart: number = getWeekStart();
  const currentDayDate: Date = getCurrentDayDate();
  const currentDayTime: number = currentDayDate.getTime();
  const dayMilliseconds: number = 24 * 60 * 60 * 1000;
  const weekdays = [
    'Monday',
    'Tuesday',
    'Wednesday',
    'Thursday',
    'Friday',
    'Saturday',
    'Sunday',
  ];

  function getWeekStart() {
    let today = new Date();

    var day = today.getDay() || 7; // Get current day number, converting Sun. to 7
    // Set the hours to day number minus 1
    //   multiplied by negative 24
    today.setHours(-24 * (day - 1), 0, 0, 0);

    return today.getTime();
  }

  function getCurrentDayDate() {
    let today = new Date();
    var day = today.getDay() || 7; // Get current day number, converting Sun. to 7
    // Set the hours to day number minus 1
    //   multiplied by negative 24
    today.setHours(0, 0, 0, 0);

    return today;
  }

  onMount(() => {
    const unsubQuery = db.subscribeQuery(
      selectedTeamState.teamDefault
        ? {
            todos: {
              $: {
                where: {
                  done: false,
                },
              },
            },
          }
        : {
            todos: {
              $: {
                where: {
                  teams: selectedTeamState.teamId,
                  done: false,
                },
              },
            },
          },
      (resp) => {
        if (resp.data) {
          const allTodos = resp.data.todos;

          let startOfDay = currentDayTime;
          let endOfDay = currentDayTime + dayMilliseconds;

          for (let i = currentDayDate.getDay(); i <= 7; i++) {
            if (i === currentDayDate.getDay()) {
              todos[i - 1] = allTodos.filter((x) => {
                return x.date === undefined || x.date < endOfDay;
              });
            } else {
              todos[i - 1] = allTodos.filter((x) => {
                return (
                  x.date !== undefined &&
                  x.date < endOfDay &&
                  x.date >= startOfDay
                );
              });
            }

            startOfDay += dayMilliseconds;
            endOfDay += dayMilliseconds;
          }
        }
      },
    );

    return () => {
      unsubQuery();
    };
  });

  async function createTodo(e: Event) {
    e.preventDefault();

    try {
      const todoId = id();

      const result = await db.transact([
        db.tx.todos[todoId].update({
          text: todoText,
          done: false,
          date: todoDate || Date.now(),
        }),
        db.tx.todos[todoId].link({
          teams: selectedTeamState.teamId,
        }),
      ]);

      console.log(result);
    } catch (e) {
      console.log(e);
    }
  }

  async function finishTodo(t: Todo) {
    try {
      const result = await db.transact([
        db.tx.todos[t.id].update({
          done: true,
        }),
      ]);

      console.log(result);
    } catch (e) {
      console.log(e);
    }
  }
</script>

<div>
  <h1>TODOs for Team: {selectedTeamState.teamName}</h1>

  <button
    type="button"
    onclick={() => {
      goto('/');
    }}
  >
    Back
  </button>

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
    >
      Create
    </button>
  </form>

  {$inspect(todos)}

  {#each todos as dayTodos, index}
    {console.log('Index', index)}
    <h2>{weekdays[index]}</h2>

    {#each dayTodos as todo}
      <div class="p-8 flex flex-col gap-4">
        <button
          type="button"
          onclick={() => {
            finishTodo(todo);
          }}
        >
          {todo.text},{todo.date},{todo.done}
        </button>
      </div>
    {/each}
  {/each}
</div>
