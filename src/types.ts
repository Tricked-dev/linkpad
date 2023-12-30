export const enum DisplayType {
  ICON = 0,
  TEXT = 1,
}

export type TextButton = {
  type: DisplayType.TEXT;
  autoSize: boolean;
  text: string;
  textSize?: number;
};
export type IconButton = {
  type: DisplayType.ICON;
  icon: string;
};

export type AnyButton = Omit<
  Partial<TextButton> & Partial<IconButton>,
  "type"
> & {
  type: DisplayType;
  id: string;
};

export type Info = {
  id: string;
  name: string;
  buttons: AnyButton[][];
}[];
