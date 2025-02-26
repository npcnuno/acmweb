import {
  edited_events,
  event_editing_store,
  unfinished_event,
} from "$lib/stores/globalStore";
import { redirect } from "@sveltejs/kit";
import type { wasmEventres } from "../../../models/product/types";
import { get } from "svelte/store";
// All svg icons used
const svg_icons = {
  dashboard:
    '<svg id="icon" style="margin: var( --side-bar-button-icon-margin );" class="w-7 h-7 text-gray-800 dark:text-white m-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><path stroke="white" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v15c0 .6.4 1 1 1h15M8 16l2.5-5.5 3 3L17.3 7 20 9.7"/></svg>',
  profile:
    '<svg style="margin: var( --side-bar-button-icon-margin );"class="w-6 h-6 text-gray-800 m-2 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8Zm-2 9a4 4 0 0 0-4 4v1c0 1.1.9 2 2 2h8a2 2 0 0 0 2-2v-1a4 4 0 0 0-4-4h-4Z" clip-rule="evenodd"/></svg>',
  users:
    '<svg style="margin: var( --side-bar-button-icon-margin );"class="w-7 h-7 text-gray-800 dark:text-white m-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M17 10v1.1l1 .5.8-.8 1.4 1.4-.8.8.5 1H21v2h-1.1l-.5 1 .8.8-1.4 1.4-.8-.8a4 4 0 0 1-1 .5V20h-2v-1.1a4 4 0 0 1-1-.5l-.8.8-1.4-1.4.8-.8a4 4 0 0 1-.5-1H11v-2h1.1l.5-1-.8-.8 1.4-1.4.8.8a4 4 0 0 1 1-.5V10h2Zm.4 3.6c.4.4.6.8.6 1.4a2 2 0 0 1-3.4 1.4A2 2 0 0 1 16 13c.5 0 1 .2 1.4.6ZM5 8a4 4 0 1 1 8 .7 7 7 0 0 0-3.3 3.2A4 4 0 0 1 5 8Zm4.3 5H7a4 4 0 0 0-4 4v1c0 1.1.9 2 2 2h6.1a7 7 0 0 1-1.8-7Z" clip-rule="evenodd"/></svg>',
  events:
    '<svg style="margin: var( --side-bar-button-icon-margin );"class="w-6 h-6 text-gray-800 dark:text-white m-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M18.5 3.1c.3.2.5.5.5.9v16a1 1 0 0 1-1.6.8L12 17V7.1l5.4-4a1 1 0 0 1 1 0ZM22 12a4 4 0 0 1-2 3.5v-7c1.2.7 2 2 2 3.5ZM10 8H4a1 1 0 0 0-1 1v6c0 .6.4 1 1 1h6V8Zm0 9H5v3c0 .6.4 1 1 1h3c.6 0 1-.4 1-1v-3Z" clip-rule="evenodd"/></svg>',
  products:
    '<svg style="margin: var( --side-bar-button-icon-margin );"class="w-6 h-6 text-gray-800 m-2 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M10 2a3 3 0 0 0-3 3v1H5a3 3 0 0 0-3 3v2.4l1.4.7a7.7 7.7 0 0 0 .7.3 21 21 0 0 0 16.4-.3l1.5-.7V9a3 3 0 0 0-3-3h-2V5a3 3 0 0 0-3-3h-4Zm5 4V5c0-.6-.4-1-1-1h-4a1 1 0 0 0-1 1v1h6Zm6.4 7.9.6-.3V19a3 3 0 0 1-3 3H5a3 3 0 0 1-3-3v-5.4l.6.3a10 10 0 0 0 .7.3 23 23 0 0 0 18-.3h.1L21 13l.4.9ZM12 10a1 1 0 1 0 0 2 1 1 0 1 0 0-2Z" clip-rule="evenodd"/></svg>',
  settings:
    '<svg style="margin: var( --side-bar-button-icon-margin );"class="w-6 h-6 text-gray-800 m-2 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24"><path d="M10.8 5a3 3 0 0 0-5.6 0H4a1 1 0 1 0 0 2h1.2a3 3 0 0 0 5.6 0H20a1 1 0 1 0 0-2h-9.2ZM4 11h9.2a3 3 0 0 1 5.6 0H20a1 1 0 1 1 0 2h-1.2a3 3 0 0 1-5.6 0H4a1 1 0 1 1 0-2Zm1.2 6H4a1 1 0 1 0 0 2h1.2a3 3 0 0 0 5.6 0H20a1 1 0 1 0 0-2h-9.2a3 3 0 0 0-5.6 0Z"/></svg>',
  payment_methods:
    '<svg style="margin: var( --side-bar-button-icon-margin );"class="w-6 h-6 m-2 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M4 5a2 2 0 0 0-2 2v10c0 1.1.9 2 2 2h16a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2H4Zm0 6h16v6H4v-6Z" clip-rule="evenodd"/><path fill-rule="evenodd" d="M5 14c0-.6.4-1 1-1h2a1 1 0 1 1 0 2H6a1 1 0 0 1-1-1Zm5 0c0-.6.4-1 1-1h5a1 1 0 1 1 0 2h-5a1 1 0 0 1-1-1Z" clip-rule="evenodd"/></svg>',
  settings_profile:
    '<svg style="margin: var( --side-bar-button-icon-margin );" class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M5 8a4 4 0 1 1 7.8 1.3l-2.5 2.5A4 4 0 0 1 5 8Zm4 5H7a4 4 0 0 0-4 4v1c0 1.1.9 2 2 2h2.2a3 3 0 0 1-.1-1.6l.6-3.4a3 3 0 0 1 .9-1.5L9 13Zm9-5a3 3 0 0 0-2 .9l-6 6a1 1 0 0 0-.3.5L9 18.8a1 1 0 0 0 1.2 1.2l3.4-.7c.2 0 .3-.1.5-.3l6-6a3 3 0 0 0-2-5Z" clip-rule="evenodd"/></svg>',
  management_admin_user:
    '<svg style="margin: var( --side-bar-button-icon-margin );" class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M8 4a4 4 0 1 0 0 8 4 4 0 0 0 0-8Zm-2 9a4 4 0 0 0-4 4v1c0 1.1.9 2 2 2h8a2 2 0 0 0 2-2v-1a4 4 0 0 0-4-4H6Zm7.3-2a6 6 0 0 0 0-6A4 4 0 0 1 20 8a4 4 0 0 1-6.7 3Zm2.2 9a4 4 0 0 0 .5-2v-1a6 6 0 0 0-1.5-4H18a4 4 0 0 1 4 4v1a2 2 0 0 1-2 2h-4.5Z" clip-rule="evenodd"/></svg>',
  management_regular_user:
    '<svg style="margin: var( --side-bar-button-icon-margin );"class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24"><path fill-rule="evenodd" d="M12 6a3.5 3.5 0 1 0 0 7 3.5 3.5 0 0 0 0-7Zm-1.5 8a4 4 0 0 0-4 4c0 1.1.9 2 2 2h7a2 2 0 0 0 2-2 4 4 0 0 0-4-4h-3Zm6.8-3.1a5.5 5.5 0 0 0-2.8-6.3c.6-.4 1.3-.6 2-.6a3.5 3.5 0 0 1 .8 6.9Zm2.2 7.1h.5a2 2 0 0 0 2-2 4 4 0 0 0-4-4h-1.1l-.5.8c1.9 1 3.1 3 3.1 5.2ZM4 7.5a3.5 3.5 0 0 1 5.5-2.9A5.5 5.5 0 0 0 6.7 11 3.5 3.5 0 0 1 4 7.5ZM7.1 12H6a4 4 0 0 0-4 4c0 1.1.9 2 2 2h.5a6 6 0 0 1 3-5.2l-.4-.8Z" clip-rule="evenodd"/></svg>',
  settings_sub_menu_color_schemme:
    '<svg style="margin: var( --side-bar-button-icon-margin );"class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24"><path d="M13.8 4.2a2 2 0 0 0-3.6 0L8.4 8.4l-4.6.3a2 2 0 0 0-1.1 3.5l3.5 3-1 4.4c-.5 1.7 1.4 3 2.9 2.1l3.9-2.3 3.9 2.3c1.5 1 3.4-.4 3-2.1l-1-4.4 3.4-3a2 2 0 0 0-1.1-3.5l-4.6-.3-1.8-4.2Z"/></svg>',
  plus_icon: `<svg style="margin: var( --side-bar-button-icon-margin );"class="w-6 h-6 text-gray-800 m-2 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24">
<path d="M 12 2 C 6.4889971 2 2 6.4889971 2 12 C 2 17.511003 6.4889971 22 12 22 C 17.511003 22 22 17.511003 22 12 C 22 6.4889971 17.511003 2 12 2 z M 12 4 C 16.430123 4 20 7.5698774 20 12 C 20 16.430123 16.430123 20 12 20 C 7.5698774 20 4 16.430123 4 12 C 4 7.5698774 7.5698774 4 12 4 z M 11 7 L 11 11 L 7 11 L 7 13 L 11 13 L 11 17 L 13 17 L 13 13 L 17 13 L 17 11 L 13 11 L 13 7 L 11 7 z"></path>
</svg>`,
  calendar: `<svg style="margin: var( --side-bar-button-icon-margin );"class="w-6 h-6 text-gray-800 bi bi-calendar-fill dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="white" viewBox="0 0 24 24">
  <path d="M3.5 0a.5.5 0 0 1 .5.5V1h8V.5a.5.5 0 0 1 1 0V1h1a2 2 0 0 1 2 2v11a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V5h16V4H0V3a2 2 0 0 1 2-2h1V.5a.5.5 0 0 1 .5-.5"/>
</svg>`,
};

// Settings sub menu display
const settings_sub_menu = [
  { type: "Previous_Menu" },
  { type: "Separator", name: "Settings" },
  {
    type: "Button",
    name: "Profile",
    icon: svg_icons.settings_profile,
    redirect: "/settings/profile",
  },
  {
    type: "Button",
    name: "Color Schemme",
    icon: svg_icons.settings_sub_menu_color_schemme,
    redirect: "/settings/color_schemme",
  },
];

export const event_sub_menu = [
  {
    type: "Button",
    name: "Go Back",
    icon: svg_icons.plus_icon,
    redirect: "/management/events",
  },
  { type: "Separator", name: "General Event Settings" },
  {
    type: "Button",
    name: get(event_editing_store!).name, // Use get() outside of reactive context
    icon: svg_icons.calendar,
    redirect: `/management/events/${get(event_editing_store!).id}`, // Template literals for dynamic values
  },
  { type: "Separator", name: "SubEvent Settings" },

  ...(get(event_editing_store!).sub_events
    ? get(event_editing_store!).sub_events.map((sub_event) => ({
        type: "Button",
        name: sub_event.price,
        icon: svg_icons.products,
        redirect: `/management/events/${get(event_editing_store!).id}/${
          sub_event.id
        }`,
      }))
    : []),
  { type: "Separator", name: "Add a SubEvent" },
  {
    type: "Button",
    name: "Add a SubEvent",
    icon: svg_icons.plus_icon,
    redirect: `/management/events/${get(event_editing_store!).id}/${
      get(event_editing_store!).sub_events?.length === undefined
        ? 0
        : get(event_editing_store!).sub_events?.length
    }`, // Adjusted to correct path
  },
];
const EventDropdown = [
  { type: "Previous_Menu" },

  { type: "Separator", name: "Manage Events" },
  {
    type: "Button",
    name: "Event List",
    icon: svg_icons.events,
    redirect: "/management/events",
  },

  //FIXME: COULD BREAK SIDE BAR IF WRONG TYPE OF EVENTS
  { type: "Separator", name: "Last Unfinished Event" },
  {
    type: "Button",
    name: get(unfinished_event!).name,
    icon: svg_icons.calendar,
    redirect: "/management/events/add",
  },

  { type: "Separator", name: "Last Edited Events" },
  get(edited_events!).map((event: wasmEventres) => ({
    type: "Button",
    name: event.name,
    icon: svg_icons.events,
    redirect: `/management/events/${event.id}`,
  })),

  { type: "Separator", name: "Add a new Event" },
  {
    type: "Button",
    name: "Add an Event",
    icon: svg_icons.plus_icon,
    redirect: "/management/events/add",
  },
];
// Management users sub menu display
const management_users_sub_menu = [
  { type: "Previous_Menu" },
  { type: "Separator", name: "Manage Users" },
  {
    type: "Button",
    name: "Administrators",
    icon: svg_icons.management_admin_user,
    redirect: "/management/users/admin",
  },
  {
    type: "Button",
    name: "Regular",
    icon: svg_icons.management_regular_user,
    redirect: "/management/users/",
  },
];

// Main menu display
export const main_menu = [
  { type: "Separator", name: "General" },
  {
    type: "Button",
    name: "Dashboard",
    icon: svg_icons.dashboard,
    redirect: "/dashboard",
  },
  {
    type: "Button",
    name: "Profile",
    icon: svg_icons.profile,
    redirect: "/profile",
  },

  { type: "Separator", name: "Management" },
  {
    type: "Drop_Side",
    name: "Users",
    icon: svg_icons.users,
    sub_menu: management_users_sub_menu,
  },
  {
    type: "Drop_Side",
    name: "Events",
    icon: svg_icons.events,
    redirect: "/management/events",
    sub_menu: event_sub_menu,
  },
  {
    type: "Button",
    name: "Products",
    icon: svg_icons.products,
    redirect: "/mangement",
  },

  { type: "Separator", name: "Configuration" },
  {
    type: "Drop_Side",
    name: "Settings",
    icon: svg_icons.settings,
    sub_menu: settings_sub_menu,
  },
  {
    type: "Button",
    name: "Payment Methods",
    icon: svg_icons.payment_methods,
    redirect: "/payment_methods",
  },
];
