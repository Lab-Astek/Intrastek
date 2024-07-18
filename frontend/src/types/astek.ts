import { UUID } from "crypto";

export type Astek = {
  id: UUID;
  indisponibilities: Indisponibility[];
};

export enum IndisponibilityType {
  Private = "Private",
}

export type Indisponibility = {
  type: IndisponibilityType;
  start: Date;
  end: Date;
};
