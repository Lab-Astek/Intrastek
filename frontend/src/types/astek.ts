import { UUID } from "crypto";
import { Interval } from "./interval";
import { Activity } from "./activity";

export type Astek = {
  id: UUID;
  indisponibilities: Indisponibility[];
  assignations: Activity[];
};

export enum IndisponibilityType {
  Private = "Private",
}

export type Indisponibility = {
  type: IndisponibilityType;
  interval: Interval;
};
