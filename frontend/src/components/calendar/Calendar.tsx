import { FC, ReactElement } from "react";
import { dayjsLocalizer } from "react-big-calendar";
import dayjs from "dayjs";
import { DatePickerEventFormData, EventFormData } from "./interfaces";

type CalendarProps = {
  title?: string;
};

const initialEventFormState: EventFormData = {
  description: "",
  todoId: undefined,
};

const initialDatePickerEventFormData: DatePickerEventFormData = {
  description: "",
  todoId: undefined,
  allDay: false,
  start: undefined,
  end: undefined,
};

const localizer = dayjsLocalizer(dayjs);

const Calendar: FC<CalendarProps> = ({ title }): ReactElement => {
  return <div></div>;
};
export default Calendar;
