import { invoke } from "@tauri-apps/api/core";

// Tax case types
export interface TaxCaseSummary {
  id: number;
  year: number;
  name: string;
  status: string;
}

export interface TaxResult {
  taxable_income: number;
  income_tax: number;
  solidarity_surcharge: number;
  church_tax: number;
  total_tax: number;
  refund_or_payment: number;
}

// Tax case commands
export const createTaxCase = (year: number, name: string) =>
  invoke<TaxCaseSummary>("create_tax_case", { year, name });

export const listTaxCases = () =>
  invoke<TaxCaseSummary[]>("list_tax_cases");

export const getTaxCase = (id: number) =>
  invoke<TaxCaseSummary>("get_tax_case", { id });

export const deleteTaxCase = (id: number) =>
  invoke<void>("delete_tax_case", { id });

// Calculation commands
export const calculateTax = (taxCaseId: number) =>
  invoke<TaxResult>("calculate_tax", { taxCaseId });
