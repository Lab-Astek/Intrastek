import { DateTimePicker } from "@mui/x-date-pickers/DateTimePicker";
import { AdapterDayjs } from "@mui/x-date-pickers/AdapterDayjs";
import { LocalizationProvider } from "@mui/x-date-pickers/LocalizationProvider";
import { Dayjs } from "dayjs";
import {
  DateTimeValidationError,
  PickerChangeHandlerContext,
} from "@mui/x-date-pickers/models";

type DatePickerProps = {
  label?: string;
  onChange?:
    | ((
        value: Dayjs | null,
        context: PickerChangeHandlerContext<DateTimeValidationError>
      ) => void)
    | undefined;
};

export default function DatePicker({ label = "", onChange }: DatePickerProps) {
  return (
    <LocalizationProvider dateAdapter={AdapterDayjs}>
      <DateTimePicker label={label} onChange={onChange} />
    </LocalizationProvider>
  );
}
