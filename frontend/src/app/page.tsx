"use client";
import { useState } from "react";
import { UUID, randomUUID } from "crypto";
import { getAstek } from "../api/asteks";
import { Astek } from "../types/astek";
import ButtonWrapper from "@/components/button";
import SelectWrapper from "@/components/inputs/select";
import { MsalProvider, useMsal } from "@azure/msal-react";
import { loginRequest } from "../authConfig";
import { PublicClientApplication } from "@azure/msal-browser";
import { msalConfig } from "../authConfig";

const TEST_ID: UUID = "2fdfd8fe-59c0-4a93-9f3b-e0f75110bb1b";
const MSAL_INSTANCE = new PublicClientApplication(msalConfig);

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
    <ButtonWrapper>
      <a href="/activity/create">Create an activity</a>
    </ButtonWrapper>
  );
}


function LoginAstekButton() {
  const { instance } = useMsal();

  function handleLogin() {
    instance.loginPopup(loginRequest).catch(e => {
      console.error(e);
    });
  }

  return (
    <button onClick={handleLogin}>Login</button>
  );
}

export default function Home() {
  return (
    <MsalProvider instance={MSAL_INSTANCE}>
      <main className="flex min-h-screen flex-col items-center justify-between p-24">
        <div className="flex items-center justify-between w-full">
          <h1>Intrastek</h1>
          <LoginAstekButton />
        </div>
        <ActivityCreationPageButton />
        <SelectWrapper label="string" value={2}>
          <p>Option 1</p>
          <p>Option 2</p>
          <p>Option 3</p>
        </SelectWrapper >
      </main>
    </MsalProvider>
  );
}
