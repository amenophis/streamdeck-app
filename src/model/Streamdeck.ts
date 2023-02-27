export class Streamdeck {
  readonly kind: string;
  readonly name: string;
  readonly serial: string;
  readonly row_count: number;
  readonly column_count: number;
  readonly key_count: number;

  constructor(kind: string, name: string, serial: string, row_count: number, column_count: number, key_count: number) {
    this.kind = kind;
    this.name = name;
    this.serial = serial;
    this.row_count = row_count;
    this.column_count = column_count;
    this.key_count = key_count;
  }
}