export interface wasmEventres {
  id: string;
  name: string;
  category: string;
  location: string;
  timedate_begin: string;
  timedate_end: string;
  subevents: { id: string }[];
}

export interface unfinished_event_struct {
  id: string;
  name: string;
  description: string;
  location: string;
  timedate_begin: string;
  timedate_end: string;
  category: string;
  sub_events: SubEvent[];
}

export interface SubEvent {
  id: string;
  instituition: string;
  parent_event: string;
  price: string;
  active: boolean;
  partner: boolean;
}

export interface sub_event_store_struct {
  parent_id: string;
  sub_events: SubEvent[];
}

export interface EventIDwasm {
  code: string;
  description: { id: string }[];
}

export interface EventID {
  id: string;
}

export interface EventRes {
  code: string;
  description: wasmEventres;
}
