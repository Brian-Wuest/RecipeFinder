/*!
//<copyright file="utilities.ts" company="Symplr"
// Copyright 2022 Symplr. All rights reserved. Confidential and Proprietary.
//</copyright>
 */
  /**
   * Calls the [checkValidity] function on a form to show html 5 form validation messages.
   * @param {string} formId - The form id to find on the page.
   */
   export function validateForm(formId: string): boolean {
    const form: HTMLFormElement = document.getElementById(formId) as HTMLFormElement;
    let test = true;

    if (form) {
      test = form.checkValidity();

      if (!test) {
        form.reportValidity();
      }

      return test;
    }

    return false;
  }
