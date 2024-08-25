import { id, init, tx } from "@instantdb/core";

export const APP_ID = "409c6ca4-bdd4-43e2-b342-ce12cc7ca281";

export interface Todo {
  id: string;
  text: string;
  done: boolean;
  createdAt: number;
}

type Schema = {
  todos: Todo;
};

export const db = init<Schema>({ appId: APP_ID });

export function addTodo(text: string) {
  return db.transact(
    tx.todos[id()].update({
      text,
      done: false,
      createdAt: Date.now(),
    }),
  );
}
