import { ICategory } from '../category';

export interface IGetCategory {
  id: number,
  name: string,
  parentCategory: ICategory
}
