import {
  useState,
  MouseEvent,
  ReactElement,
  FC,
  Dispatch,
  SetStateAction,
} from "react";
import {
  Box,
  Button,
  ButtonGroup,
  Card,
  CardContent,
  CardHeader,
  Container,
  Divider,
} from "@mui/material";

import { Calendar, type Event, dateFnsLocalizer } from "react-big-calendar";

import format from "date-fns/format";
import parse from "date-fns/parse";
import startOfWeek from "date-fns/startOfWeek";
import getDay from "date-fns/getDay";
import enUS from "date-fns/locale/en-US";

import "react-big-calendar/lib/css/react-big-calendar.css";

import EventInfo from "./EventInfo";
import AddEventModal from "./AddEventModal";
import EventInfoModal from "./EventInfoModal";
import AddDatePickerEventModal from "./AddDatePickerEventModal";
import { IndisponibilityType } from "@/types/astek";

const locales = {
  "en-US": enUS,
};

const localizer = dateFnsLocalizer({
  format,
  parse,
  startOfWeek,
  getDay,
  locales,
});

export interface IEventInfo extends Event {
  _id: string;
  description: string;
  type: IndisponibilityType;
}

export interface EventFormData {
  description: string;
  todoId?: string;
}

export interface DatePickerEventFormData {
  description: string;
  allDay: boolean;
  start?: Date;
  end?: Date;
}

export const generateId = () =>
  (Math.floor(Math.random() * 10000) + 1).toString();

const initialEventFormState: EventFormData = {
  description: "",
  todoId: undefined,
};

const initialDatePickerEventFormData: DatePickerEventFormData = {
  description: "",
  allDay: false,
  start: undefined,
  end: undefined,
};

type EventCalendarProps = {
  onAddEvent: (
    event: Event,
    eventFormData: EventFormData,
    events: [IEventInfo[], Dispatch<SetStateAction<IEventInfo[]>>]
  ) => void;
  onDeleteEvent?: (event: Event) => void;

  eventHandlers?: [IEventInfo[], Dispatch<SetStateAction<IEventInfo[]>>];
};

const EventCalendar: FC<EventCalendarProps> = ({
  onAddEvent,
  onDeleteEvent,
  eventHandlers,
}: EventCalendarProps): ReactElement => {
  const [openSlot, setOpenSlot] = useState(false);
  const [openDatepickerModal, setOpenDatepickerModal] = useState(false);
  const [currentEvent, setCurrentEvent] = useState<Event | IEventInfo | null>(
    null
  );

  const [eventInfoModal, setEventInfoModal] = useState(false);

  const [events, setEvents] = eventHandlers || useState<IEventInfo[]>([]);

  const [eventFormData, setEventFormData] = useState<EventFormData>(
    initialEventFormState
  );

  const [datePickerEventFormData, setDatePickerEventFormData] =
    useState<DatePickerEventFormData>(initialDatePickerEventFormData);

  const handleSelectSlot = (event: Event) => {
    setOpenSlot(true);
    setCurrentEvent(event);
  };

  const handleSelectEvent = (event: IEventInfo) => {
    setCurrentEvent(event);
    setEventInfoModal(true);
  };

  const handleClose = () => {
    setEventFormData(initialEventFormState);
    setOpenSlot(false);
  };

  const handleDatePickerClose = () => {
    setDatePickerEventFormData(initialDatePickerEventFormData);
    setOpenDatepickerModal(false);
  };

  const onAddEventInternal = (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    if (!currentEvent) return;
    if (!currentEvent.start || !currentEvent.end) return;

    onAddEvent(currentEvent, eventFormData, [events, setEvents]);

    handleClose();
  };

  const onAddEventFromDatePicker = (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    const addHours = (date: Date | undefined, hours: number) => {
      return date ? date.setHours(date.getHours() + hours) : undefined;
    };

    const setMinToZero = (date: any) => {
      date.setSeconds(0);

      return date;
    };

    const data: IEventInfo = {
      ...datePickerEventFormData,
      _id: generateId(),
      start: setMinToZero(datePickerEventFormData.start),
      end: datePickerEventFormData.allDay
        ? addHours(datePickerEventFormData.start, 12)
        : setMinToZero(datePickerEventFormData.end),
      type: IndisponibilityType.Private,
    };

    const newEvents = [...events, data];

    setEvents(newEvents);
    setDatePickerEventFormData(initialDatePickerEventFormData);
  };

  const onDeleteEventInternal = () => {
    currentEvent && onDeleteEvent && onDeleteEvent(currentEvent);
    setEvents(() =>
      [...events].filter((e) => e._id !== (currentEvent as IEventInfo)._id!)
    );
    setEventInfoModal(false);
  };

  return (
    <Box
      mt={2}
      mb={2}
      component="main"
      sx={{
        flexGrow: 1,
        py: 8,
      }}
    >
      <Container maxWidth={false}>
        <Card>
          <CardHeader
            title="Calendar"
            subheader="Manage your indisponibilities and assignations here."
          />
          <Divider />
          <CardContent>
            <Box sx={{ display: "flex", justifyContent: "space-between" }}>
              <ButtonGroup
                size="large"
                variant="contained"
                aria-label="outlined primary button group"
              >
                <Button
                  onClick={() => setOpenDatepickerModal(true)}
                  size="small"
                  variant="contained"
                >
                  Add event
                </Button>
              </ButtonGroup>
            </Box>
            <Divider style={{ margin: 10 }} />
            <AddEventModal
              open={openSlot}
              handleClose={handleClose}
              eventFormData={eventFormData}
              setEventFormData={setEventFormData}
              onAddEvent={onAddEventInternal}
            />
            <AddDatePickerEventModal
              open={openDatepickerModal}
              handleClose={handleDatePickerClose}
              datePickerEventFormData={datePickerEventFormData}
              setDatePickerEventFormData={setDatePickerEventFormData}
              onAddEvent={onAddEventFromDatePicker}
            />
            <EventInfoModal
              open={eventInfoModal}
              handleClose={() => setEventInfoModal(false)}
              onDeleteEvent={onDeleteEventInternal}
              currentEvent={currentEvent as IEventInfo}
            />
            <Calendar
              localizer={localizer}
              events={events}
              onSelectEvent={handleSelectEvent}
              onSelectSlot={handleSelectSlot}
              selectable
              startAccessor="start"
              components={{ event: EventInfo }}
              endAccessor="end"
              defaultView="week"
              eventPropGetter={(event) => {
                return {
                  style: {
                    backgroundColor:
                      event.type === IndisponibilityType.Private
                        ? "red"
                        : "#b64fc8",
                    borderColor:
                      event.type === IndisponibilityType.Private
                        ? "red"
                        : "#b64fc8",
                  },
                };
              }}
              style={{
                height: 500,
                width: 1000,
              }}
            />
          </CardContent>
        </Card>
      </Container>
    </Box>
  );
};

export default EventCalendar;
