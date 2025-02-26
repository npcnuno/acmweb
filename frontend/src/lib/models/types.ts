import { postLabel } from "./enums";

export interface post {
  title: string,
  img: string,
  description: string,
  status: postLabel,
  link: string,
}
