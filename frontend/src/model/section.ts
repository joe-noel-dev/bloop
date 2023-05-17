import {Entity} from './entity';

export interface Section extends Entity {
  name: string;
  start: number;
  loop: boolean;
}
