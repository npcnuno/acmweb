import {
  boolean,
  minLength,
  number,
  object,
  optional,
  string,
  date,
  isoDateTime,
  array,
  any,
} from "valibot";

export const _productSchema = object({
  id: string([minLength(0, "ERROR: Event ID is required.")]),
  name: string([minLength(0, "A product must have a name")]),
  category: string([minLength(0, "A product must have a category")]),
  location: string([minLength(0, "A product must have a location")]),
  issuer: string([minLength(0, "")]),
  left: number(),
  sold: number(),
  time_begin: date(),
  time_end: date(),
  description: optional(string()),
});

export const _subEventSchema = object({
  id: string([minLength(0, "ERROR: Event ID is required.")]),
  parent_event: string([minLength(0, "ERROR: Event ID is required.")]),
  institution: string([minLength(0, "ERROR: Event ID is required.")]),
  partner: optional(boolean()),
  active: optional(boolean()),
  price: string([minLength(0, "ERROR: a price is required.")]),
});

export const _createEventSchema = object({
  name: string([minLength(0, "An Event Must Have a Name")]),
  description: string(),
  location: string([minLength(0, "An Event Must Have a Location")]),
  timedate_begin: string([isoDateTime()]),
  timedate_end: string([isoDateTime()]),
  category: string([minLength(0, "ERROR: Event Must Have a Category")]),
  sub_events: optional(array(_subEventSchema)),
});
