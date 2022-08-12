import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { UserDataService } from 'src/app/services/data/user-data.service';

@Component({
  selector: 'app-recipe-search',
  templateUrl: './recipe-search.component.html',
  styleUrls: ['./recipe-search.component.scss']
})
export class RecipeSearchComponent implements OnInit {
  loading = false;

  constructor(private currentRoute: ActivatedRoute) { }

  ngOnInit(): void {
    // Determines if recipes should only be loaded for the current user.
    const forCurrentUser = this.currentRoute.snapshot.queryParamMap.get('currentUser')?.toLowerCase() === 'true';

    this.loading = true;
  }
}
