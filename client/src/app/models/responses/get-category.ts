import { ICategory } from '../category';

export interface IGetCategoryResponse {
  id: number,
  name: string,
  parentCategory: ICategory
}
