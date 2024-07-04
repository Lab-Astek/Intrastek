"use client";
import { useState } from "react";
import { UUID, randomUUID } from "crypto";
import { getAstek } from "../api/asteks";
import { Astek } from "../types/astek";
import BasicButton from "@/components/button";
import SelectInput from "@/components/inputs/select";

const TEST_ID: UUID = "2fdfd8fe-59c0-4a93-9f3b-e0f75110bb1b";

function AstekButton() {
  let [result, setResult] = useState<Astek>({ id: randomUUID(), indisponibilities: [], assignations: [] });

  function handleClick() {

    getAstek(TEST_ID)
      .then((response) => {
        setResult(response.data);
      })
      .catch((error) => {
        console.log(error);
      });

  }

  return (
    <button onClick={handleClick}>
      Click me {result.id}
    </button>
  );
}

function ActivityCreationPageButton() {
  return (
    <BasicButton>
      <a href="/activity/create">Create an activity</a>
    </BasicButton>
  );
}

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <h1>Intrastek</h1>
      <ActivityCreationPageButton />
      <SelectInput>
        <p>Option 1</p>
        <p>Option 2</p>
        <p>Option 3</p>
      </SelectInput>
    </main>
  );
}
