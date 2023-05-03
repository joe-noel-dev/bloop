import {Entity} from './entity';

export interface Section extends Entity {
  name: string;
  start: number;
  beatLength: number;
  loop: boolean;
}
