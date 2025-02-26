import {
  boolean,
  email,
  minLength,
  number,
  object,
  optional,
  string,
  date,
  regex,
} from "valibot";

export const _userSchema = object({
  id: string([minLength(0, "ID is required.")]),
  name: string([minLength(1, "Please enter the Student name.")]),
  email: string([
    minLength(1, "Please enter the student institutional email."),
    email("The email address is badly formatted."),
  ]),
  password: optional(string()),
  course: string([minLength(1, "Student must have a course")]),
  role: optional(string([])),
  locker: optional(string([minLength(1, "Please type the locker code")])),
  phone: string([
    minLength(9, "Invalid phone Number!"),
    regex(
      /^[+]*[(]{0,1}[0-9]{1,4}[)]{0,1}[-\s\./0-9]*$/,
      "Invalid Phone Number",
    ),
  ]),
  is_partner: boolean(),
  instutuition: optional(string([])),
});

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
  partner: boolean(),
  active: boolean(),
  price: string([minLength(0, "ERROR: a price is required.")]),
});
