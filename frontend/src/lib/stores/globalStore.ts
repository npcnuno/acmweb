import { writable } from "svelte/store";
import type {
  SubEvent,
  sub_event_store_struct,
  unfinished_event_struct,
} from "../../models/product/types";
import { browser } from "$app/environment";

export const authToken = writable("");

export const refreshToken = writable("");

//FIXME: IT COULD BE BETTER BUR FUCK IT. I COULD MAYBE DO ONE STORE FOR EVERYTHING ALTHOUGH IF THE BACK-END DOESNT RETURN AN ID OF THE CREATED EVENT I CAN'T MAKE IT WORK
export const edited_events = persistStore<[]>("edited_events", [{}]);

//FIXME: CREATE A STORE WITH THE instituition NAME OR CODE SO IT WOULD BE SAFER, THE BACK END SOULDN'T USE THE NAME BUT A CODE WHEN THE USER LOGS IN
export const event_editing_store = persistStore<sub_event_store_struct>(
  "event_editing_store",
  {
    id: "add",
    name: "test",
    description: "",
    location: "",
    timedate_begin: "",
    timedate_end: "",
    category: "",
    sub_events: [],
  },
);
//FIXME: GETTING CONFUSED, WHAT WOULD BE THE BEST SOLUTION TO CACHE EVENTS IN SVELTE, KNOWING...
//OK I THOUGH OF A SOLUTION THE BEST WOULD BE TO HAVE A CACHED SUBEVENTS, I NEED TO CACHE IT IN A WAY THAT SUPERFORMS CAN ACCESS IT,
//SO MAYBE ALL I NEED TO DO IS CREATE A STORE FOR SUBEVENT BEING EDIDTED THAT IS A SIMPLE ARRAY, THAT IS SAVED BY
export const unfinished_event = persistStore<unfinished_event_struct>(
  "unfinished_event",
  {
    id: "add",
    name: "NONE",
    description: "",
    location: "",
    timedate_begin: "",
    timedate_end: "",
    category: "",
    sub_events: [],
  },
);

export const unfinished_sub_event = persistStore<SubEvent[]>(
  "unfinished_sub_event_store",
  [{}],
);

function persistStore<T>(key: string, initialValue: any): any {
  const store = writable<T>(initialValue); // Start with initialValue

  if (typeof window !== "undefined") {
    const storedValue = localStorage.getItem(key);
    if (storedValue !== null) {
      store.set(JSON.parse(storedValue)); // Only parse if not null
    }

    let timeout: number;

    store.subscribe((value) => {
      clearTimeout(timeout);
      timeout = setTimeout(() => {
        localStorage.setItem(key, JSON.stringify(value));
      }, 20); // 1 minute delay
    });
  }

  return store;
}
