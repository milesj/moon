import React from 'react';
import { faSpaceStationMoon } from '@fortawesome/pro-duotone-svg-icons';
import Icon from '../../../ui/iconography/Icon';
import DocLink from '../../../ui/typography/Link';
import CTA from '../../Home/CTA';
import Screenshots from './Screenshots';

export default function Hero() {
	return (
		<div className="bg-gradient-to-b from-slate-900 to-slate-600 star-pattern">
			<div className="max-w-7xl mx-auto py-10 px-4 pb-6 sm:py-12 sm:px-6 md:py-14 lg:py-16 lg:px-8 xl:py-20 flex flex-col md:flex-row">
				<div className="text-center lg:text-left md:w-6/12">
					<h1 className="text-6xl tracking-tight font-extrabold text-purple-600">
						<img src="/img/logo-hero.svg" alt="moonbase" title="moonbase" width={300} />
					</h1>

					<p className="mt-1 mb-0 text-base text-white sm:text-lg sm:max-w-xl sm:mx-auto md:text-xl lg:mx-0 md:w-[80%]">
						A service for monitoring your codebases and scaling their CI pipelines.
					</p>

					<p className="mt-1 text-white opacity-50 text-sm md:text-base md:pr-4">
						For{' '}
						<DocLink href="/moon" variant="muted">
							moon
						</DocLink>{' '}
						powered repositories.
					</p>

					<div className="mt-3 flex justify-center lg:justify-start">
						<div>
							<CTA href="https://moonrepo.app">
								Try it today
								<Icon
									icon={faSpaceStationMoon}
									className="ml-1 md:ml-2 inline-block"
									style={{ maxWidth: 18 }}
								/>
							</CTA>
						</div>
					</div>
				</div>

				<div className="mt-4 md:mt-0 md:w-6/12 relative">
					<Screenshots />
				</div>
			</div>
		</div>
	);
}
