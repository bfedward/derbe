import { ArrowRight, CalendarDays, Check, UsersRound } from "lucide-react";
import { Link } from "react-router-dom";
import { Button } from "@/components/ui/button";

const benefits = [
  {
    icon: CalendarDays,
    title: "Leave without the guesswork",
    description: "See requests, decisions, and team availability in one place.",
  },
  {
    icon: UsersRound,
    title: "People stay in the picture",
    description: "Keep employee details and reporting lines easy to manage.",
  },
  {
    icon: Check,
    title: "Clear decisions, less chasing",
    description: "Give employees and managers a shared view of what happens next.",
  },
];

export function PublicHomePage() {
  return (
    <div className="min-h-screen overflow-hidden bg-[#f6f5ef] text-[#17231f]">
      <header className="mx-auto flex max-w-7xl items-center justify-between px-5 py-5 sm:px-8">
        <Link to="/" className="flex items-center gap-2 text-xl font-bold tracking-tight">
          <span className="flex size-9 items-center justify-center rounded-xl bg-[#1f5b49] text-sm text-white">
            d
          </span>
          derbe
        </Link>
        <nav className="flex items-center gap-3">
          <Link
            to="/login"
            className="hidden text-sm font-medium text-[#4d5b55] sm:inline-flex"
          >
            Log in
          </Link>
          <Button asChild className="rounded-full bg-[#1f5b49] px-5 hover:bg-[#174536]">
            <Link to="/login">
              Get started <ArrowRight className="ml-2 size-4" />
            </Link>
          </Button>
        </nav>
      </header>

      <main>
        <section className="mx-auto grid max-w-7xl items-center gap-12 px-5 pb-20 pt-12 sm:px-8 lg:grid-cols-[1.05fr_0.95fr] lg:gap-16 lg:pb-28 lg:pt-20">
          <div className="relative z-10">
            <p className="mb-5 inline-flex items-center gap-2 rounded-full border border-[#d3ddd4] bg-white/70 px-4 py-2 text-xs font-semibold uppercase tracking-[0.16em] text-[#28634f]">
              People-first leave management
            </p>
            <h1 className="max-w-2xl text-5xl font-semibold leading-[1.04] tracking-[-0.055em] sm:text-6xl lg:text-7xl">
              Time off should feel <span className="text-[#328265]">simple.</span>
            </h1>
            <p className="mt-6 max-w-xl text-lg leading-8 text-[#58645e]">
              Derbe gives your team one calm place to request leave, make decisions, and
              plan around the days that matter.
            </p>
            <div className="mt-9 flex flex-wrap items-center gap-4">
              <Button
                asChild
                size="lg"
                className="h-12 rounded-full bg-[#1f5b49] px-6 text-base hover:bg-[#174536]"
              >
                <Link to="/login">
                  Open Derbe <ArrowRight className="ml-2 size-4" />
                </Link>
              </Button>
              <span className="text-sm text-[#69756f]">
                A clearer way to manage time away.
              </span>
            </div>
            <div className="mt-10 flex items-center gap-3 text-sm text-[#58645e]">
              <div className="flex -space-x-2" aria-hidden="true">
                <span className="size-8 rounded-full border-2 border-[#f6f5ef] bg-[#e6b798]" />
                <span className="size-8 rounded-full border-2 border-[#f6f5ef] bg-[#8db5a2]" />
                <span className="size-8 rounded-full border-2 border-[#f6f5ef] bg-[#d9c27f]" />
              </div>
              <span>Made for teams that work better together</span>
            </div>
          </div>

          <div className="relative mx-auto w-full max-w-xl">
            <div className="absolute -right-12 -top-16 size-64 rounded-full bg-[#e6d9aa]/60 blur-3xl" />
            <div className="absolute -bottom-12 -left-10 size-56 rounded-full bg-[#b9d7c7]/70 blur-3xl" />
            <div className="relative rounded-[2rem] border border-white/80 bg-white/75 p-4 shadow-[0_30px_90px_-45px_rgba(32,66,51,0.4)] backdrop-blur sm:p-6">
              <div className="flex items-center justify-between border-b border-[#edf0eb] pb-5">
                <div>
                  <p className="text-xs font-medium uppercase tracking-[0.14em] text-[#86918b]">
                    Your team
                  </p>
                  <p className="mt-1 text-lg font-semibold">A little room to recharge</p>
                </div>
                <div className="flex size-10 items-center justify-center rounded-full bg-[#e9f2ec] text-[#28634f]">
                  <CalendarDays className="size-5" />
                </div>
              </div>
              <div className="mt-5 grid grid-cols-7 gap-2 text-center text-xs">
                {["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].map((day) => (
                  <span key={day} className="py-2 font-medium text-[#a0aaa4]">
                    {day[0]}
                  </span>
                ))}
                {Array.from({ length: 28 }, (_, index) => {
                  const day = index + 1;
                  const away = [5, 6, 12, 13, 19, 20].includes(day);
                  return (
                    <span
                      key={day}
                      className={`flex aspect-square items-center justify-center rounded-xl ${away ? "bg-[#dcece2] font-semibold text-[#28634f]" : "text-[#65716a]"}`}
                    >
                      {day}
                    </span>
                  );
                })}
              </div>
              <div className="mt-5 space-y-3 rounded-2xl bg-[#f5f7f2] p-4">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-sm font-semibold">Coming up</p>
                    <p className="mt-1 text-xs text-[#78847d]">Your team this week</p>
                  </div>
                  <span className="rounded-full bg-white px-3 py-1 text-xs font-medium text-[#28634f]">
                    3 away
                  </span>
                </div>
                <div
                  className="flex -space-x-2"
                  role="img"
                  aria-label="Three teammates are away"
                >
                  <span className="size-9 rounded-full border-2 border-[#f5f7f2] bg-[#d7a68d]" />
                  <span className="size-9 rounded-full border-2 border-[#f5f7f2] bg-[#8eb5a4]" />
                  <span className="size-9 rounded-full border-2 border-[#f5f7f2] bg-[#d8c175]" />
                  <span className="flex size-9 items-center justify-center rounded-full border-2 border-[#f5f7f2] bg-white text-[10px] font-medium text-[#66736b]">
                    +2
                  </span>
                </div>
              </div>
              <div className="absolute -left-7 top-24 hidden rounded-2xl border border-[#e9eee8] bg-white p-4 shadow-lg sm:block">
                <div className="flex items-center gap-3">
                  <span className="flex size-9 items-center justify-center rounded-xl bg-[#edf5ef] text-[#328265]">
                    <Check className="size-4" />
                  </span>
                  <div>
                    <p className="text-xs font-semibold">Request approved</p>
                    <p className="mt-1 text-[11px] text-[#87918b]">All sorted</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>

        <section className="border-y border-[#e5e7df] bg-white/55">
          <div className="mx-auto max-w-7xl px-5 py-16 sm:px-8 lg:py-20">
            <div className="max-w-2xl">
              <p className="text-xs font-semibold uppercase tracking-[0.16em] text-[#328265]">
                The everyday, made easier
              </p>
              <h2 className="mt-3 text-3xl font-semibold tracking-tight sm:text-4xl">
                Good teams need room to plan.
              </h2>
              <p className="mt-4 text-base leading-7 text-[#69756f]">
                From a first request to the team calendar, keep the details clear and the
                process moving.
              </p>
            </div>
            <div className="mt-10 grid gap-4 md:grid-cols-3">
              {benefits.map(({ icon: Icon, title, description }) => (
                <article
                  key={title}
                  className="rounded-3xl border border-[#e8ebe4] bg-white p-6"
                >
                  <span className="flex size-11 items-center justify-center rounded-2xl bg-[#edf5ef] text-[#328265]">
                    <Icon className="size-5" />
                  </span>
                  <h3 className="mt-5 text-lg font-semibold">{title}</h3>
                  <p className="mt-2 text-sm leading-6 text-[#69756f]">{description}</p>
                </article>
              ))}
            </div>
          </div>
        </section>
      </main>

      <footer className="mx-auto flex max-w-7xl flex-col gap-4 px-5 py-8 text-sm text-[#758078] sm:flex-row sm:items-center sm:justify-between sm:px-8">
        <Link to="/" className="font-semibold text-[#263d32]">
          derbe
        </Link>
        <span>Make time away easier for everyone.</span>
        <Link to="/login" className="font-medium text-[#28634f]">
          Log in
        </Link>
      </footer>
    </div>
  );
}
