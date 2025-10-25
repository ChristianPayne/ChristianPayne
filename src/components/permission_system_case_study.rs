use leptos::prelude::*;

#[component]
pub fn PermissionSystemCaseStudy() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <div class="container mx-auto max-w-7xl">
                // Hero Section
                <div class="py-10 md:py-20">
                    <div class="text-center">
                        <h1 class="text-4xl md:text-6xl font-bold mb-2">"Permission System"</h1>
                        <p class="text-lg text-gray-600 mb-6">"Model Match"</p>
                        <p class="text-lg md:text-xl mb-8 max-w-3xl mx-auto">
                            "A scalable permission management solution for Model Match that combines role-based access control, granular permissions, and real-time updates to provide precise control over user capabilities."
                        </p>
                        <div class="flex flex-wrap justify-center gap-4 mb-8">
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "TypeScript"
                            </span>
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "DynamoDB"
                            </span>
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "AppSync"
                            </span>
                        </div>
                    </div>
                </div>

                // Project Overview
                <section class="py-10 md:py-20">
                    <div class="max-w-4xl mx-auto">
                        <h2 class="text-4xl font-bold mb-6">"Project Overview"</h2>
                        <p class="text-lg mb-4">
                            "Developed as a core component of Model Match, this permission system was created to handle the complex access control needs of a multi-tenant platform. The initial system was too restrictive, applying the same access levels to all users within a tenant. This new system provides user-level control while maintaining the multi-tenant structure, allowing different users to have different access levels. All changes are applied instantly, and the flexible role system handles access management for users at every level of the organization."
                        </p>
                    </div>
                </section>

                // History & Motivation
                <section class="py-10 md:py-20">
                    <div class="max-w-4xl mx-auto">
                        <h2 class="text-4xl font-bold mb-6">"History & Motivation"</h2>
                        <p class="text-lg mb-4">
                            "As Model Match expanded beyond individual loan officers to serve entire mortgage companies and real estate teams, companies reached out with needs for different access levels. Loan officers needed to see agent data and Market Insights, while business development teams needed broader market intelligence. The existing permission system couldn't handle these industry-specific role distinctions."
                        </p>
                        <p class="text-lg mb-4">
                            "Beyond simple on/off switches, the team realized that many features needed granular configuration. Users might need access to Market Insights but with limited data fields, or agents might not want access to loan data. The existing system couldn't handle these nuanced permission requirements."
                        </p>
                        <p class="text-lg mb-4">
                            "Additionally, some companies didn't want to pay for all the data features, so the team needed to section them off with their own sub-feature flags. The new permission system could control access to specific data sets, Market Insights, and tools, allowing organizations to build the right access levels for their team structure and budget."
                        </p>
                        <p class="text-lg">
                            "This led to a redesign focused on mortgage industry workflows. The new system could handle the complex permission needs of mortgage companies, from individual loan officers to enterprise teams, while maintaining the platform's core value proposition and enabling flexible pricing tiers."
                        </p>
                    </div>
                </section>

                // Technical Architecture
                <section class="py-10 md:py-20">
                    <h2 class="text-4xl font-bold text-center mb-12">"Technical Architecture"</h2>
                    <div class="max-w-4xl mx-auto">
                        <p class="text-lg mb-8">
                            "Understanding how the permission system works requires diving into its technical architecture. Here we'll explore the building blocks and see how they come together in the management interface."
                        </p>
                        <img
                            src="/images/permissions.png"
                            alt="Model Match permission management interface showing tenant access controls"
                            class="w-full rounded-lg shadow-lg object-cover mb-8"
                        />
                    </div>

                    <div class="max-w-4xl mx-auto">
                        <h3 class="text-2xl font-bold mb-4">
                            "Database Storage & On-Demand Compilation"
                        </h3>
                        <p class="text-lg mb-6">
                            "Each permission layer is stored independently in the database, containing only the values that that specific layer cares about. This approach optimizes storage and allows for efficient updates without affecting other layers."
                        </p>

                        <div class="space-y-6">
                            <div class="rounded-xl p-4 md:m-6 border bg-white shadow-sm min-w-0">
                                <h4 class="text-xl font-bold mb-3">"Layer-Specific Storage"</h4>
                                <div class="space-y-4">
                                    <div>
                                        <h5 class="font-medium mb-2">
                                            "Code Defaults (Hardcoded):"
                                        </h5>
                                        <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                            <code class="language-typescript text-sm">
                                                {concat!(
                                                    "// Stored in TypeScript files, not database\n",
                                                    "const CODE_DEFAULTS = {\n",
                                                    "  \"userManagement\": {\n",
                                                    "    \"createUser\": false,\n",
                                                    "    \"editProfile\": false\n",
                                                    "  }\n",
                                                    "};",
                                                )}
                                            </code>
                                        </div>
                                    </div>

                                    <div>
                                        <h5 class="font-medium mb-2">
                                            "Role Permissions (Database):"
                                        </h5>
                                        <div class="rounded bg-gray-100 p-4 mb-4">
                                            <code class="language-typescript text-sm whitespace-pre-line">
                                                {concat!(
                                                    "// Only stores overrides, not complete structure\n",
                                                    "const rolePermissions = {\n",
                                                    "  \"userManagement\": {\n",
                                                    "    \"createUser\": true  // Only the changed value\n",
                                                    "  }\n",
                                                    "  // editProfile not stored - uses code default\n",
                                                    "};",
                                                )}
                                            </code>
                                        </div>
                                    </div>

                                    <div class="hidden">
                                        <h5 class="font-medium mb-2">
                                            "User Permissions (Database):"
                                        </h5>
                                        <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                            <code class="language-typescript text-sm">
                                                {concat!(
                                                    "// Minimal storage - only user-specific overrides\n",
                                                    "const userPermissions = {\n",
                                                    "  \"userManagement\": {\n",
                                                    "    \"editProfile\": true  // User-specific override\n",
                                                    "  }\n",
                                                    "  // createUser not stored - inherits from role\n",
                                                    "};",
                                                )}
                                            </code>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="rounded-xl p-4 md:m-6 border bg-white shadow-sm min-w-0">
                                <h4 class="text-xl font-bold mb-3">"On-Demand Compilation"</h4>
                                <p class="mb-4">
                                    "When a client requests their permissions, the system performs a real-time merge of all layers. This compilation happens on-demand, ensuring clients always receive the most up-to-date permission state without requiring pre-computation or caching."
                                </p>
                                <div class="space-y-4">
                                    <div class="hidden">
                                        <h5 class="font-medium mb-2">"Compilation Process:"</h5>
                                        <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                            <code class="language-typescript text-sm">
                                                {concat!(
                                                    "// Happens when client requests permissions\n",
                                                    "function compileUserPermissions(userId: string) {\n",
                                                    "  const codeDefaults = getCodeDefaults();\n",
                                                    "  const rolePermissions = getRolePermissions(userId);\n",
                                                    "  const userPermissions = getUserPermissions(userId);\n",
                                                    "  \n",
                                                    "  // Merge all layers in priority order\n",
                                                    "  return mergePermissionsWithPriority([\n",
                                                    "    codeDefaults,\n",
                                                    "    rolePermissions,\n",
                                                    "    userPermissions\n",
                                                    "  ]);\n",
                                                    "}",
                                                )}
                                            </code>
                                        </div>
                                    </div>
                                </div>
                                <p class="mt-4 text-gray-500">
                                    "This approach provides several benefits: efficient storage by only storing deltas and the ability to modify any layer without affecting others."
                                </p>
                            </div>
                        </div>
                    </div>
                </section>

                // Security & Implementation
                <section class="py-10 md:py-20">
                    <div class="max-w-4xl mx-auto">
                        <div class="space-y-8">
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"AWS Cognito Integration"</h3>
                                <p class="text-lg">
                                    <span>
                                        "AWS Cognito provides backup protection for the permission system. When someone becomes a Tenant Admin, they're automatically added to the "
                                    </span>
                                    <code class="bg-gray-100 px-2 py-1 rounded text-sm">
                                        "tenantAdmin"
                                    </code>
                                    <span>
                                        " AWS Cognito group. This creates an additional safety layer at the routing level, ensuring that company settings are protected by both the permission system and AWS Cognito's route guards."
                                    </span>
                                </p>
                            </div>
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Value Access"</h3>
                                <p class="text-lg mb-4">
                                    <span>
                                        "Permissions are organized in a tree structure, with each permission's location defined by a key reducer - an array of strings that represents an arbitrary location in the permission tree. This allows for precise targeting of permissions at any level, from broad feature access to specific actions within features. For example, a key reducer like "
                                    </span>
                                    <code class="bg-gray-100 px-2 py-1 rounded text-sm">
                                        "['core', 'contacts', 'delete']"
                                    </code>
                                    <span>
                                        " can control the ability to delete contacts, while "
                                    </span>
                                    <code class="bg-gray-100 px-2 py-1 rounded text-sm">
                                        "['core', 'contacts', 'view']"
                                    </code>
                                    <span>" controls viewing access."</span>
                                </p>
                                <p class="text-lg">
                                    "Sometimes layers of the permission system may not have a specified value for a particular location, so the key reducer method may not find anything there. This is expected behavior - the system gracefully handles missing permission definitions by falling back to code defaults, ensuring there's always a defined permission state."
                                </p>
                            </div>
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Role-Based Strategy"</h3>
                                <p class="text-lg mb-4">
                                    "Roles were added to the system so permissions could be defined independently from users. When there's a group of users that commonly have the same role in a company, you can add the permissions onto the role and assign roles (or multiple roles) to users."
                                </p>
                                <p class="text-lg mb-4">
                                    "Roles have a hierarchy that can be changed on the permissions page to give you control over cascading. When two or more roles have the same permissions specified, we would have a conflict. The order you put the roles in on the permissions system management page allows us to know how merge conflicts should be handled."
                                </p>
                                <p class="text-lg">
                                    "User-specific permissions are treated as the most specific level, used when a single user needs unique access that doesn't fit into any existing role structure. This approach was chosen for its maintainability and scalability - when permissions are stored in roles, adding the same permissions to a new user is as simple as assigning the role, and updating permissions for multiple users requires changing just the role definition."
                                </p>
                            </div>
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Cascading Permissions"</h3>
                                <p class="text-lg mb-6">
                                    "Permissions cascade through the system like a waterfall, flowing from the most general level down to the most specific. We took inspiration from CSS in this regard. This means you can set broad permissions at the tenant level and then override them with more specific settings at the role or user level. The system automatically resolves conflicts by using the most specific permission value it finds, ensuring predictable behavior."
                                </p>
                                <p class="text-lg mb-6">
                                    "The permission hierarchy consists of five distinct levels, each with its own purpose and priority. Understanding how these levels interact is key to effectively managing permissions across the platform."
                                </p>
                                <div class="space-y-6">
                                    <div class="rounded-xl p-4 md:m-6 border bg-white shadow-sm min-w-0">
                                        <h4 class="text-xl font-bold mb-3">
                                            "Permission Hierarchy"
                                        </h4>
                                        <div class="space-y-4">
                                            <div class="flex items-start">
                                                <span class="mr-3 mt-2">"-"</span>
                                                <div>
                                                    <p class="font-medium">"Tenant Access"</p>
                                                    <p class="text-sm text-gray-500">
                                                        "The master switch that can override all other permissions. If a permission is set to false here, it will be false regardless of other settings. Think of it as a circuit breaker on the outside of a house - if this turns off, everything downstream of it also gets shut off."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="flex items-start">
                                                <span class="mr-3 mt-2">"-"</span>
                                                <div>
                                                    <p class="font-medium">"Industry Defaults"</p>
                                                    <p class="text-sm text-gray-500">
                                                        "Industry-specific permission overrides that can modify the base permissions for particular industries. Sometimes we work with other industries or we might in the future. This was mainly a future-proofing step that gave us code defaults for whole industry tenant access. Some features, like loan data, just are not relevant in some industries, so we can turn them off for groups of tenants without having to go into each tenant and tune industry-specific features."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="border-t border-gray-200 my-4"></div>
                                            <div class="flex items-start">
                                                <span class="mr-3 mt-2">"-"</span>
                                                <div>
                                                    <p class="font-medium">"User-Specific Permissions"</p>
                                                    <p class="text-sm text-gray-500">
                                                        "Individual user permissions that can override role-based permissions, used for exceptional cases where a user needs unique access. This level allows administrators to grant specific permissions to individual users that don't fit into any existing role structure. These permissions are the most specific and take precedence over role-based permissions."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="flex items-start">
                                                <span class="mr-3 mt-2">"-"</span>
                                                <div>
                                                    <p class="font-medium">"Role-Based Permissions"</p>
                                                    <p class="text-sm text-gray-500">
                                                        "Permissions assigned through roles, with the Tenant Admin role having the highest priority, followed by user-defined roles in their specified order. The system includes static roles like Tenant Admin and Everyone, plus user-defined roles that can be created to match organizational needs. When multiple roles have conflicting permissions, the order specified on the permissions management page determines which one takes precedence."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="flex items-start">
                                                <span class="mr-3 mt-2">"-"</span>
                                                <div>
                                                    <p class="font-medium">"Code Defaults"</p>
                                                    <p class="text-sm text-gray-500">
                                                        "Base permissions baked into the code that serve as the default floor. These are used when no other permission level has explicitly set a value, ensuring there's always a defined permission state. This level acts as the final fallback in the permission hierarchy, guaranteeing that every permission check returns a definitive true or false value."
                                                    </p>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Recursive Implementation
                <section class="py-10 md:py-20">
                    <h2 class="text-4xl font-bold text-center mb-12">"Recursive Implementation"</h2>
                    <div class="max-w-4xl mx-auto">
                        <p class="text-lg mb-8">
                            "The permission system leverages recursion throughout its implementation, making it highly flexible and future-proof. This approach allows the system to handle permissions of any depth without requiring changes to the core functionality."
                        </p>
                        <div class="space-y-8">
                            <div class="rounded-xl p-4 md:m-6 border bg-white shadow-sm min-w-0">
                                <h3 class="text-xl font-bold mb-4">"Recursive Types"</h3>
                                <p class="mb-4">
                                    "The type system uses recursion to automatically generate types for the permission system. The `DeepPartial` type makes all keys optional, while `DeepPermissionsDetails` automatically converts boolean permissions into detailed permission objects."
                                </p>

                                <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                    <code class="language-typescript text-sm">
                                        {concat!(
                                            "// A recursive type that makes all nested fields optional\n",
                                            "export type DeepPartial<T> = {\n",
                                            "  [P in keyof T]?: T[P] extends object\n",
                                            "    ? DeepPartial<T[P]>\n",
                                            "    : T[P];\n",
                                            "};",
                                        )}
                                    </code>
                                </div>

                                <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                    <code class="language-typescript text-sm">
                                        {concat!(
                                            "// A recursive type that renders all nested fields to be optional.\n",
                                            "export type DeepPermissionsDetails<T> = ",
                                            "T extends boolean\n",
                                            "  ? PermissionDetails\n",
                                            "  : T extends object\n",
                                            "  ? { [Key in keyof T]: DeepPermissionsDetails<T[Key]> }\n",
                                            "  : T;",
                                        )}
                                    </code>
                                </div>

                                <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                    <code class="language-typescript text-sm">
                                        {concat!(
                                            "// Override type for permission updates\n",
                                            "export type PermissionsOverride = DeepPartial<Permissions>;",
                                        )}
                                    </code>
                                </div>

                                <p>
                                    "With this approach, developers can focus on defining the base permission structure, and the type system automatically handles the creation of all necessary types, reducing the chance of type mismatches."
                                </p>
                            </div>
                            <div class="rounded-xl p-4 md:m-6 border bg-white shadow-sm min-w-0">
                                <h3 class="text-xl font-bold mb-4">"Recursive UI"</h3>
                                <p class="mb-4">
                                    "The permission management interface uses a recursive component that can render permissions of any depth. Each level of the component handles both boolean values (permission toggles) and nested objects (permission groups), automatically creating new levels as needed."
                                </p>
                                <p>
                                    "This approach means that adding new permissions or deeper nesting in the future requires no changes to the UI code - the same component handles all levels of the permission tree."
                                </p>

                                <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                    <code class="language-typescript text-sm">
                                        {concat!(
                                            "/** A helper type to require some information about a specific permission. */\n",
                                            "type PermissionDetails = {\n",
                                            "    display_name: string;\n",
                                            "    description?: string;\n",
                                            "    /** Defines whether or not this setting is shown on the company settings page for the tenant admin to control. */\n",
                                            "    isCompanySetting?: boolean;\n",
                                            "    /** Defines whether or not this setting is for tenant admins specifically. These settings are for controlling the access that the tenant admin has, not controlling what is in their control. */\n",
                                            "    isTenantAdminSetting?: boolean;\n",
                                            "    /** Flags this setting as experimental. */\n",
                                            "    experimental?: boolean;\n",
                                            "    icon?: string;\n",
                                            "}",
                                        )}
                                    </code>
                                </div>
                            </div>
                            <div class="rounded-xl p-4 md:m-6 border bg-white shadow-sm min-w-0">
                                <h3 class="text-xl font-bold mb-4">"Permission Merging"</h3>
                                <p class="mb-4">
                                    "The core of the permission system is the recursive merging function that combines permissions from different sources. This function can handle permissions of any depth, automatically merging them based on priority while maintaining the structure of the permission tree."
                                </p>
                                <p class="mb-6">
                                    "Built-in safeguards prevent infinite recursion, and the system tracks the depth of merging to ensure it doesn't exceed safe limits. This makes the system both powerful and safe, capable of handling complex permission scenarios while preventing potential issues."
                                </p>

                                <h4 class="text-lg font-semibold mb-3">
                                    "Input Permissions (Priority: Later = Higher Priority)"
                                </h4>
                                <p class="mb-4 text-gray-500">
                                    "Multiple permission objects are provided in priority order, with later permissions taking precedence over earlier ones."
                                </p>

                                <div class="space-y-4 mb-6">
                                    <div class="hidden">
                                        <h5 class="font-medium mb-2">
                                            "Manager Role Permissions (Lowest Priority):"
                                        </h5>
                                        <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                            <code class="language-typescript text-sm">
                                                {concat!(
                                                    "{\n",
                                                    "  \"userManagement\": {\n",
                                                    "    \"createUser\": true,\n",
                                                    "    \"editProfile\": false\n",
                                                    "  },\n",
                                                    "  \"systemSettings\": {\n",
                                                    "    \"modifyConfig\": false\n",
                                                    "  }\n",
                                                    "}",
                                                )}
                                            </code>
                                        </div>
                                    </div>

                                    <div class="hidden">
                                        <h5 class="font-medium mb-2">
                                            "Admin Role Permissions (Medium Priority):"
                                        </h5>
                                        <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                            <code class="language-typescript text-sm">
                                                {concat!(
                                                    "{\n",
                                                    "  \"userManagement\": {\n",
                                                    "    \"createUser\": false,    // Override manager\n",
                                                    "    \"editProfile\": true     // Override manager\n",
                                                    "  },\n",
                                                    "  \"systemSettings\": {\n",
                                                    "    \"modifyConfig\": true,    // Override manager\n",
                                                    "    \"backupData\": false      // New permission\n",
                                                    "  }\n",
                                                    "}",
                                                )}
                                            </code>
                                        </div>
                                    </div>

                                    <div>
                                        <h5 class="font-medium mb-2">
                                            "SuperAdmin Role Permissions (Highest Priority):"
                                        </h5>
                                        <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                            <code class="language-typescript text-sm">
                                                {concat!(
                                                    "{\n",
                                                    "  \"userManagement\": {\n",
                                                    "    \"createUser\": true       // Override admin\n",
                                                    "  },\n",
                                                    "  \"systemSettings\": {\n",
                                                    "    \"viewLogs\": false,       // New permission\n",
                                                    "    \"backupData\": true       // Override admin\n",
                                                    "  }\n",
                                                    "}",
                                                )}
                                            </code>
                                        </div>
                                    </div>
                                </div>

                                <h4 class="text-lg font-semibold mb-3">
                                    "Recursive Merge Process"
                                </h4>
                                <p class="mb-4 text-gray-500">
                                    "The function recursively walks through each permission object, merging them with priority-based resolution while maintaining the nested structure."
                                </p>

                                <div class="space-y-4">
                                    <div class="hidden">
                                        <h5 class="font-medium mb-2">
                                            "Code Defaults (Base Layer):"
                                        </h5>
                                        <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                            <code class="language-typescript text-sm">
                                                {concat!(
                                                    "// All possible permissions defined as defaults\n",
                                                    "const CODE_DEFAULTS = {\n",
                                                    "  \"userManagement\": {\n",
                                                    "    \"createUser\": false,\n",
                                                    "    \"editProfile\": false,\n",
                                                    "    \"deleteUser\": false,\n",
                                                    "    \"resetPassword\": false\n",
                                                    "  },\n",
                                                    "  \"systemSettings\": {\n",
                                                    "    \"modifyConfig\": false,\n",
                                                    "    \"viewLogs\": false,\n",
                                                    "    \"backupData\": false,\n",
                                                    "    \"restoreData\": false\n",
                                                    "  }\n",
                                                    "}",
                                                )}
                                            </code>
                                        </div>
                                    </div>

                                    <div>
                                        <h5 class="font-medium mb-2">
                                            "Start with code defaults:"
                                        </h5>
                                        <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                            <code class="language-typescript text-sm">
                                                "Result = CODE_DEFAULTS"
                                            </code>
                                        </div>
                                    </div>

                                    <div>
                                        <h5 class="font-medium mb-2">
                                            "Merge Manager → Admin → SuperAdmin (recursive depth tracking):"
                                        </h5>
                                        <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                            <code class="language-typescript text-sm">
                                                {concat!(
                                                    "Result = {\n",
                                                    "  \"userManagement\": {\n",
                                                    "    \"createUser\": true,     // SuperAdmin overrides Admin overrides Manager\n",
                                                    "    \"editProfile\": true,     // Admin overrides Manager\n",
                                                    "    \"deleteUser\": false,    // Kept from code defaults\n",
                                                    "    \"resetPassword\": true    // Added by Admin\n",
                                                    "  },\n",
                                                    "  \"systemSettings\": {\n",
                                                    "    \"modifyConfig\": true,    // Admin overrides Manager\n",
                                                    "    \"viewLogs\": false,       // Overridden by SuperAdmin\n",
                                                    "    \"backupData\": true,      // SuperAdmin overrides Admin\n",
                                                    "    \"restoreData\": false     // Kept from code defaults\n",
                                                    "  }\n",
                                                    "}",
                                                )}
                                            </code>
                                        </div>
                                    </div>
                                </div>

                                <h4 class="text-lg font-semibold mb-3">
                                    "Permission Validation & Client Delivery"
                                </h4>
                                <p class="mb-4 text-gray-500">
                                    "After merging all permission sources, the system validates the final result and ensures every possible permission is explicitly defined before sending to the client."
                                </p>

                                <div class="space-y-4">
                                    <div class="hidden">
                                        <h5 class="font-medium mb-2">"Final Client Payload:"</h5>
                                        <div class="rounded bg-gray-100 p-4 mb-4 whitespace-pre-wrap">
                                            <code class="language-typescript text-sm">
                                                {concat!(
                                                    "// Complete, validated permissions sent to client\n",
                                                    "const clientPermissions = {\n",
                                                    "  \"userManagement\": {\n",
                                                    "    \"createUser\": true,\n",
                                                    "    \"editProfile\": true,\n",
                                                    "    \"deleteUser\": false,\n",
                                                    "    \"resetPassword\": true\n",
                                                    "  },\n",
                                                    "  \"systemSettings\": {\n",
                                                    "    \"modifyConfig\": true,\n",
                                                    "    \"viewLogs\": false,\n",
                                                    "    \"backupData\": true,\n",
                                                    "    \"restoreData\": false\n",
                                                    "  }\n",
                                                    "};",
                                                )}
                                            </code>
                                        </div>
                                    </div>
                                </div>

                                <p class="mt-6 text-gray-500">
                                    "This validation ensures that the client always receives a complete, consistent permission object with every possible permission explicitly defined. No permission checks will fail due to missing or undefined values - the system guarantees that every permission has a definitive true or false value."
                                </p>
                            </div>
                        </div>
                    </div>
                </section>

                // Real-Time Updates
                <section class="py-10 md:py-20">
                    <h2 class="text-4xl font-bold text-center mb-12">
                        "Real-Time Permission Updates"
                    </h2>
                    <div class="max-w-4xl mx-auto">
                        <p class="text-lg mb-8">
                            "The permission system doesn't just compile permissions on-demand - it also provides real-time updates to ensure users always have the most current access levels without requiring page refreshes."
                        </p>

                        <div class="space-y-8">
                            <div class="rounded-xl p-4 md:m-6 border bg-white shadow-sm min-w-0">
                                <h3 class="text-xl font-bold mb-4">
                                    "Live Permission Synchronization"
                                </h3>
                                <p class="mb-4">
                                    "When any permission layer is updated (role changes, user-specific overrides, or tenant-wide settings), the system automatically recompiles permissions and pushes the complete, updated permission list to all affected clients."
                                </p>

                                <div class="space-y-4">
                                    <div>
                                        <h4 class="font-medium mb-2">"Update Flow:"</h4>
                                        <div class="space-y-2 text-sm">
                                            <div class="flex items-center">
                                                <span class="mr-3">"-"</span>
                                                <span>
                                                    "Permission layer is modified (role, user, or tenant level)"
                                                </span>
                                            </div>
                                            <div class="flex items-center">
                                                <span class="mr-3">"-"</span>
                                                <span>
                                                    "System identifies all users affected by the change"
                                                </span>
                                            </div>
                                            <div class="flex items-center">
                                                <span class="mr-3">"-"</span>
                                                <span>
                                                    "Permissions are recompiled for each affected user"
                                                </span>
                                            </div>
                                            <div class="flex items-center">
                                                <span class="mr-3">"-"</span>
                                                <span>
                                                    "Updated permissions are pushed to all connected clients"
                                                </span>
                                            </div>
                                            <div class="flex items-center">
                                                <span class="mr-3">"-"</span>
                                                <span>
                                                    "User interface updates instantly with new functionality"
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <p class="mt-6">
                                    "This real-time synchronization means that when an administrator grants or revokes permissions, users see the changes immediately. Features appear and disappear from their interface without any manual refresh, creating a seamless experience where permission changes take effect instantly across the entire platform."
                                </p>
                            </div>

                            <div class="rounded-xl p-4 md:m-6 border bg-white shadow-sm min-w-0">
                                <h3 class="text-xl font-bold mb-4">"Instant User Experience"</h3>
                                <p class="mb-4">
                                    "The real-time updates create a dynamic user experience where the interface adapts to permission changes as they happen. Users don't need to refresh their browser or navigate away and back to see updated functionality."
                                </p>

                                <div class="grid md:grid-cols-2 gap-6">
                                    <div>
                                        <h4 class="font-medium mb-2">"Permission Granted:"</h4>
                                        <ul class="text-sm space-y-1 text-gray-500">
                                            <li>"• New menu items appear instantly"</li>
                                            <li>"• Previously buttons become visible"</li>
                                            <li>"• Additional form fields are enabled"</li>
                                            <li>"• New navigation options are added"</li>
                                        </ul>
                                    </div>
                                    <div>
                                        <h4 class="font-medium mb-2">"Permission Revoked:"</h4>
                                        <ul class="text-sm space-y-1 text-gray-500">
                                            <li>"• Menu items disappear immediately"</li>
                                            <li>"• Buttons become disabled or"</li>
                                            <li>"• Form fields are locked or removed"</li>
                                            <li>"• Navigation options are removed"</li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Disclaimer
                <section class="py-10 md:py-20">
                    <div class="max-w-4xl mx-auto">
                        <div class="rounded-xl p-4 md:m-6 border bg-white shadow-sm min-w-0">
                            <h3 class="text-2xl font-bold mb-4">"About These Examples"</h3>
                            <p class="text-lg mb-4">
                                "The permission examples shown throughout this case study are illustrative concepts designed to demonstrate the system's architecture and functionality. They do not represent actual permission values or structures used in Model Match's private codebase."
                            </p>
                            <p class="text-lg">
                                "The core concepts remain accurate: recursive type definitions, cascading permission hierarchies, delta-based storage, and on-demand compilation. However, the specific permission names, values, and organizational structure have been simplified for clarity and to protect proprietary information."
                            </p>
                        </div>
                    </div>
                </section>
            </div>
        </div>
    }
}
