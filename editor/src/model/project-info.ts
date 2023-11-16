import {Entity} from './entity';

export interface ProjectInfo extends Entity {
  name: string;
  version: string;
  lastSaved: number;
}
