import { UUID } from "crypto";

export interface ITodo {
  _id: UUID;
  title: string;
  color?: string;
}

export interface IEventInfo extends Event {
  _id: UUID;
  description: string;
  todoId?: UUID;
}

export interface EventFormData {
  description: string;
  todoId?: UUID;
}

export interface DatePickerEventFormData {
  description: string;
  todoId?: UUID;
  allDay: boolean;
  start?: Date;
  end?: Date;
}
