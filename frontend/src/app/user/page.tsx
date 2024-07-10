"use client";
import { addIndisponibility, getAstek } from "@/api/asteks";
import EventCalendar, {
  EventFormData,
  IEventInfo,
} from "@/components/calendar/EventCalendar";
import Page from "@/components/page";
import { Astek, Indisponibility, IndisponibilityType } from "@/types/astek";
import { UUID } from "crypto";
import { Dispatch, SetStateAction, useEffect, useState } from "react";

import { type Event } from "react-big-calendar";

const TEST_ID: UUID = "2fdfd8fe-59c0-4a93-9f3b-e0f75110bb1b";

export default function Home() {
  let [result, setResult] = useState<Astek | undefined>(undefined);
  let [baseEvents, setBaseEvents] = useState<IEventInfo[]>([]);
  let [started, setStarted] = useState<boolean>(false);

  const onAddEvent = (
    event: Event,
    eventFormData: EventFormData,
    [events, setEvents]: [IEventInfo[], Dispatch<SetStateAction<IEventInfo[]>>]
  ) => {
    if (!event.start || !event.end) {
      return;
    }

    addIndisponibility(TEST_ID, {
      type: IndisponibilityType.Private,
      interval: { start: event.start, end: event.end },
    }).then((response) => {
      setEvents([
        ...events,
        {
          ...eventFormData,
          _id: response.data.toString(),
          start: event?.start,
          end: event?.end,
          type: IndisponibilityType.Private,
        },
      ]);
    });
  };

  getAstek(TEST_ID)
    .then((response) => {
      setResult(response.data);
    })
    .catch((error) => {
      console.log(error);
    });

  useEffect(() => {
    getAstek(TEST_ID)
      .then((response) => {
        let atk: Astek = response.data;

        let events: IEventInfo[] = atk.indisponibilities.map((indis, idx) => {
          console.log(indis);
          return {
            _id: idx.toString(),
            description: idx.toString(),
            start: new Date(indis.interval.start),
            end: new Date(indis.interval.end),
            type: indis.type,
          };
        });
      })
      .catch((error) => {
        console.log(error);
      });
  }, []);

  return (
    <Page title="Your calendar">
      <EventCalendar
        eventHandlers={[baseEvents, setBaseEvents]}
        onAddEvent={onAddEvent}
      />
      <h1>Astek:{result?.id}</h1>
      <h2>Indisponibilities:</h2>
      <ul>
        {result?.indisponibilities.map((indis, idx) => (
          <li key={idx}>{JSON.stringify(indis)}</li>
        ))}
      </ul>
    </Page>
  );
}
